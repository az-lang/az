use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::keywords::{
    CONDITIONAL_ALTERNATIVE_OPENER, CONDITIONAL_ANTECEDENT_OPENER,
};
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::block::{
    Block, BlockContentsValidationError, BlockPositionsValidationError,
};
use super::expression::{
    Expression, ExpressionContentsValidationError,
    ExpressionPositionsValidationError,
};
use super::to_reduced_first_token_content::ToReducedFirstTokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct BidirectionalConditional<StringType> {
    pub antecedent: Box<Expression<StringType>>,
    pub consequent: Block<StringType>,
    pub alternative: Box<Expression<StringType>>,
    pub antecedent_opener_position: SubstringPosition,
    pub alternative_opener_position: SubstringPosition,
    pub antecedent_opener_fillers: FillerVec<StringType>,
    pub alternative_opener_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> BidirectionalConditional<StringType>
where
    for<'a> TokenContent<&'a str>: std::fmt::Display + PartialEq,
{
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<BidirectionalConditionalContentsValidationError>> {
        let mut errors = vec![];
        if let Err(antecedent_opener_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.antecedent_opener_fillers)
        {
            errors.extend(antecedent_opener_filler_errors.into_iter().map(|error| {
                BidirectionalConditionalContentsValidationError(
                    BidirectionalConditionalContentsValidationErrorKind::AntecedentOpenerFiller(
                        error,
                    ),
                )
            }));
        }
        {
            const ANTECEDENT_OPENER_TOKEN_CONTENT: TokenContent<&str> =
                TokenContent::Identifier(CONDITIONAL_ANTECEDENT_OPENER);
            let antecedent_opener_conflicts_with_antecedent =
                match TokenCollection::try_from(
                    format!(
                        "{}{}",
                        ANTECEDENT_OPENER_TOKEN_CONTENT,
                        self.antecedent
                            .as_ref()
                            .to_reduced_first_token_content()
                    )
                    .as_str(),
                ) {
                    Ok(tokens) => {
                        !tokens.into_iter().next().is_some_and(|token| {
                            token.content == ANTECEDENT_OPENER_TOKEN_CONTENT
                        })
                    }
                    Err(_) => true,
                };
            if antecedent_opener_conflicts_with_antecedent {
                errors.push(BidirectionalConditionalContentsValidationError(
                    BidirectionalConditionalContentsValidationErrorKind::Antecedent(
                        AntecedentValidationErrorKind::OpenerLexicalConflict,
                    ),
                ));
            }
            if let Err(antecedent_errors) =
                self.antecedent.validate_contents_impl()
            {
                errors.extend(antecedent_errors.into_iter().map(|error| {
                    BidirectionalConditionalContentsValidationError(
                        BidirectionalConditionalContentsValidationErrorKind::Antecedent(
                            AntecedentValidationErrorKind::Contents(Box::new(
                                error,
                            )),
                        ),
                    )
                }));
            }
        }
        if let Err(consequent_errors) =
            self.consequent.validate_contents_impl()
        {
            errors.extend(consequent_errors.into_iter().map(|error| {
                BidirectionalConditionalContentsValidationError(
                    BidirectionalConditionalContentsValidationErrorKind::Consequent(error),
                )
            }));
        }
        if let Err(filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.alternative_opener_fillers)
        {
            errors.extend(filler_errors.into_iter().map(|error| {
                BidirectionalConditionalContentsValidationError(
                    BidirectionalConditionalContentsValidationErrorKind::AlternativeOpenerFiller(
                        error,
                    ),
                )
            }));
        }
        {
            const ALTERNATIVE_OPENER_TOKEN_CONTENT: TokenContent<&str> =
                TokenContent::Identifier(CONDITIONAL_ALTERNATIVE_OPENER);
            let alternative_opener_conflicts_with_alternative =
                match TokenCollection::try_from(
                    format!(
                        "{}{}",
                        ALTERNATIVE_OPENER_TOKEN_CONTENT,
                        self.alternative.to_reduced_first_token_content()
                    )
                    .as_str(),
                ) {
                    Ok(tokens) => {
                        !tokens.into_iter().next().is_some_and(|token| {
                            token.content == ALTERNATIVE_OPENER_TOKEN_CONTENT
                        })
                    }
                    Err(_) => true,
                };
            if alternative_opener_conflicts_with_alternative {
                errors.push(BidirectionalConditionalContentsValidationError(
                    BidirectionalConditionalContentsValidationErrorKind::Alternative(
                        AlternativeValidationErrorKind::OpenerLexicalConflict,
                    ),
                ));
            }
            if let Err(alternative_errors) =
                self.alternative.validate_contents_impl()
            {
                errors.extend(alternative_errors.into_iter().map(|error| {
                    BidirectionalConditionalContentsValidationError(
                        BidirectionalConditionalContentsValidationErrorKind::Alternative(
                            AlternativeValidationErrorKind::Contents(
                                Box::new(error),
                            ),
                        ),
                    )
                }));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size>
    BidirectionalConditional<StringType>
{
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<
        BidirectionalConditionalPositionsValidationError,
    > {
        validate_filler_vec_positions(
            &self.antecedent_opener_fillers,
            expected_start_character_position,
        )
            .map_error(|error| {
                BidirectionalConditionalPositionsValidationError(
                    BidirectionalConditionalPositionsValidationErrorKind::AntecedentOpenerFiller(error),
                )
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    CONDITIONAL_ANTECEDENT_OPENER,
                    &self.antecedent_opener_position,
                    expected_start_character_position,
                )
                .map_error(|error| {
                    BidirectionalConditionalPositionsValidationError(
                        BidirectionalConditionalPositionsValidationErrorKind::AntecedentOpener(error),
                    )
                })
            })
            .merge_with(|expected_start_character_position| {
                self.antecedent
                    .validate_positions_impl(
                        expected_start_character_position,
                    )
                    .map_error(|error| {
                        BidirectionalConditionalPositionsValidationError(
                            BidirectionalConditionalPositionsValidationErrorKind::Antecedent(
                                Box::new(error),
                            ),
                        )
                    })
            })
            .merge_with(|expected_start_character_position| {
                self.consequent
                    .validate_positions_impl(
                        expected_start_character_position,
                    )
                    .map_error(|error| {
                        BidirectionalConditionalPositionsValidationError(
                            BidirectionalConditionalPositionsValidationErrorKind::Consequent(
                                error
                            ),
                        )
                    })
            })
            .merge_with(|expected_start_character_position| {
                validate_filler_vec_positions(
                    &self.alternative_opener_fillers,
                    expected_start_character_position,
                )
                    .map_error(|error| {
                        BidirectionalConditionalPositionsValidationError(
                            BidirectionalConditionalPositionsValidationErrorKind::AlternativeOpenerFiller(error),
                        )
                    })
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    CONDITIONAL_ALTERNATIVE_OPENER,
                    &self.alternative_opener_position,
                    expected_start_character_position,
                )
                .map_error(|error| {
                    BidirectionalConditionalPositionsValidationError(
                        BidirectionalConditionalPositionsValidationErrorKind::AlternativeOpener(error),
                    )
                })
            })
            .merge_with(|expected_start_character_position| {
                self.alternative
                    .validate_positions_impl(
                        expected_start_character_position,
                    )
                    .map_error(|error| {
                        BidirectionalConditionalPositionsValidationError(
                            BidirectionalConditionalPositionsValidationErrorKind::Alternative(
                                Box::new(error),
                            ),
                        )
                    })
            })
    }
}

impl<StringType> BidirectionalConditional<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.antecedent_opener_fillers
            .first()
            .map(|filler| filler.position.start)
            .unwrap_or(self.antecedent_opener_position.start)
    }
}

#[derive(Debug)]
pub(super) struct BidirectionalConditionalContentsValidationError(
    BidirectionalConditionalContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct BidirectionalConditionalPositionsValidationError(
    BidirectionalConditionalPositionsValidationErrorKind,
);

#[derive(Debug)]
enum AlternativeValidationErrorKind {
    Contents(Box<ExpressionContentsValidationError>),
    OpenerLexicalConflict,
}

#[derive(Debug)]
enum AntecedentValidationErrorKind {
    Contents(Box<ExpressionContentsValidationError>),
    OpenerLexicalConflict,
}

#[derive(Debug)]
enum BidirectionalConditionalContentsValidationErrorKind {
    Alternative(AlternativeValidationErrorKind),
    AlternativeOpenerFiller(FillerVecContentsValidationError),
    Antecedent(AntecedentValidationErrorKind),
    AntecedentOpenerFiller(FillerVecContentsValidationError),
    Consequent(BlockContentsValidationError),
}

#[derive(Debug)]
enum BidirectionalConditionalPositionsValidationErrorKind {
    Alternative(Box<ExpressionPositionsValidationError>),
    AlternativeOpener(PositionsValidationError),
    AlternativeOpenerFiller(FillerVecPositionsValidationError),
    Antecedent(Box<ExpressionPositionsValidationError>),
    AntecedentOpener(PositionsValidationError),
    AntecedentOpenerFiller(FillerVecPositionsValidationError),
    Consequent(BlockPositionsValidationError),
}

impl std::fmt::Display for AlternativeValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            AlternativeValidationErrorKind::Contents(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            AlternativeValidationErrorKind::OpenerLexicalConflict => {
                write!(
                    formatter,
                    "alternative opener lexically conflicts with the alternative"
                )
            }
        }
    }
}

impl std::fmt::Display for AntecedentValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            AntecedentValidationErrorKind::Contents(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            AntecedentValidationErrorKind::OpenerLexicalConflict => {
                write!(
                    formatter,
                    "antecedent opener lexically conflicts with the antecedent"
                )
            }
        }
    }
}

impl std::fmt::Display for BidirectionalConditionalContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            BidirectionalConditionalContentsValidationErrorKind::Alternative(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalContentsValidationErrorKind::AlternativeOpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalContentsValidationErrorKind::Antecedent(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalContentsValidationErrorKind::AntecedentOpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalContentsValidationErrorKind::Consequent(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for BidirectionalConditionalPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            BidirectionalConditionalPositionsValidationErrorKind::Alternative(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalPositionsValidationErrorKind::AlternativeOpener(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalPositionsValidationErrorKind::AlternativeOpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalPositionsValidationErrorKind::Antecedent(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalPositionsValidationErrorKind::AntecedentOpener(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalPositionsValidationErrorKind::AntecedentOpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BidirectionalConditionalPositionsValidationErrorKind::Consequent(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for BidirectionalConditionalContentsValidationError {}

impl std::error::Error for BidirectionalConditionalPositionsValidationError {}

impl<StringType, TokenStringType: From<&'static str>> Tokenize<TokenStringType>
    for BidirectionalConditional<StringType>
where
    Block<StringType>: Tokenize<TokenStringType>,
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens = self
            .antecedent_opener_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        tokens.push(Token {
            content: TokenContent::Identifier(TokenStringType::from(
                CONDITIONAL_ANTECEDENT_OPENER,
            )),
            position: self.antecedent_opener_position,
        });
        tokens.extend(self.antecedent.tokenize());
        tokens.extend(self.consequent.tokenize());
        tokens.extend(
            self.alternative_opener_fillers.into_iter().map(Into::into),
        );
        tokens.push(Token {
            content: TokenContent::Identifier(TokenStringType::from(
                CONDITIONAL_ALTERNATIVE_OPENER,
            )),
            position: self.alternative_opener_position,
        });
        tokens.extend(self.alternative.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_bidirectional_conditional_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<BidirectionalConditional<$source_string_type>>
            for BidirectionalConditional<$target_string_type>
        {
            fn from(
                value: BidirectionalConditional<$source_string_type>,
            ) -> Self {
                BidirectionalConditional {
                    antecedent: Box::new((*value.antecedent).into()),
                    consequent: value.consequent.into(),
                    alternative: Box::new((*value.alternative).into()),
                    antecedent_opener_position: value
                        .antecedent_opener_position,
                    alternative_opener_position: value
                        .alternative_opener_position,
                    antecedent_opener_fillers: value
                        .antecedent_opener_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    alternative_opener_fillers: value
                        .alternative_opener_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_bidirectional_conditional_string_type_conversion!(&str, Arc<str>);
impl_bidirectional_conditional_string_type_conversion!(&str, Box<str>);
impl_bidirectional_conditional_string_type_conversion!(&str, Rc<str>);
impl_bidirectional_conditional_string_type_conversion!(&str, String);
impl_bidirectional_conditional_string_type_conversion!(Box<str>, Arc<str>);
impl_bidirectional_conditional_string_type_conversion!(Box<str>, Rc<str>);
impl_bidirectional_conditional_string_type_conversion!(Box<str>, String);
impl_bidirectional_conditional_string_type_conversion!(String, Arc<str>);
impl_bidirectional_conditional_string_type_conversion!(String, Box<str>);
impl_bidirectional_conditional_string_type_conversion!(String, Rc<str>);
