use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::keywords::CONDITIONAL_ANTECEDENT_OPENER;
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
pub struct UnidirectionalConditional<StringType> {
    pub antecedent: Box<Expression<StringType>>,
    pub consequent: Block<StringType>,
    pub opener_position: SubstringPosition,
    pub opener_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> UnidirectionalConditional<StringType>
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
    ) -> Result<(), Vec<UnidirectionalConditionalContentsValidationError>>
    {
        let mut errors = vec![];
        if let Err(opener_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.opener_fillers)
        {
            errors.extend(opener_filler_errors.into_iter().map(|error| {
                UnidirectionalConditionalContentsValidationError(
                    UnidirectionalConditionalContentsValidationErrorKind::OpenerFiller(
                        error,
                    ),
                )
            }));
        }
        {
            const OPENER_TOKEN_CONTENT: TokenContent<&str> =
                TokenContent::Identifier(CONDITIONAL_ANTECEDENT_OPENER);
            let opener_conflicts_with_antecedent =
                match TokenCollection::try_from(
                    format!(
                        "{}{}",
                        OPENER_TOKEN_CONTENT,
                        self.antecedent
                            .as_ref()
                            .to_reduced_first_token_content()
                    )
                    .as_str(),
                ) {
                    Ok(tokens) => {
                        !tokens.into_iter().next().is_some_and(|token| {
                            token.content == OPENER_TOKEN_CONTENT
                        })
                    }
                    Err(_) => true,
                };
            if opener_conflicts_with_antecedent {
                errors.push(UnidirectionalConditionalContentsValidationError(
                    UnidirectionalConditionalContentsValidationErrorKind::Antecedent(
                        AntecedentValidationErrorKind::OpenerLexicalConflict,
                    ),
                ));
            }
            if let Err(antecedent_errors) =
                self.antecedent.validate_contents_impl()
            {
                errors.extend(antecedent_errors.into_iter().map(|error| {
                    UnidirectionalConditionalContentsValidationError(
                        UnidirectionalConditionalContentsValidationErrorKind::Antecedent(
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
                UnidirectionalConditionalContentsValidationError(
                    UnidirectionalConditionalContentsValidationErrorKind::Consequent(error),
                )
            }));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size>
    UnidirectionalConditional<StringType>
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
        UnidirectionalConditionalPositionsValidationError,
    > {
        validate_filler_vec_positions(
            &self.opener_fillers,
            expected_start_character_position,
        )
            .map_error(|error| {
                UnidirectionalConditionalPositionsValidationError(
                    UnidirectionalConditionalPositionsValidationErrorKind::OpenerFiller(error),
                )
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    CONDITIONAL_ANTECEDENT_OPENER,
                    &self.opener_position,
                    expected_start_character_position,
                )
                .map_error(|error| {
                    UnidirectionalConditionalPositionsValidationError(
                        UnidirectionalConditionalPositionsValidationErrorKind::Opener(error),
                    )
                })
            })
            .merge_with(|expected_start_character_position| {
                self.antecedent
                    .validate_positions_impl(
                        expected_start_character_position,
                    )
                    .map_error(|error| {
                        UnidirectionalConditionalPositionsValidationError(
                            UnidirectionalConditionalPositionsValidationErrorKind::Antecedent(
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
                        UnidirectionalConditionalPositionsValidationError(
                            UnidirectionalConditionalPositionsValidationErrorKind::Consequent(
                                error
                            ),
                        )
                    })
            })
    }
}

impl<StringType> UnidirectionalConditional<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.opener_fillers
            .first()
            .map(|filler| filler.position.start)
            .unwrap_or(self.opener_position.start)
    }
}

#[derive(Debug)]
pub(super) struct UnidirectionalConditionalContentsValidationError(
    UnidirectionalConditionalContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct UnidirectionalConditionalPositionsValidationError(
    UnidirectionalConditionalPositionsValidationErrorKind,
);

#[derive(Debug)]
enum AntecedentValidationErrorKind {
    Contents(Box<ExpressionContentsValidationError>),
    OpenerLexicalConflict,
}

#[derive(Debug)]
enum UnidirectionalConditionalContentsValidationErrorKind {
    Antecedent(AntecedentValidationErrorKind),
    Consequent(BlockContentsValidationError),
    OpenerFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
enum UnidirectionalConditionalPositionsValidationErrorKind {
    Antecedent(Box<ExpressionPositionsValidationError>),
    Consequent(BlockPositionsValidationError),
    Opener(PositionsValidationError),
    OpenerFiller(FillerVecPositionsValidationError),
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
                    "opener lexically conflicts with the antecedent"
                )
            }
        }
    }
}

impl std::fmt::Display for UnidirectionalConditionalContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            UnidirectionalConditionalContentsValidationErrorKind::Antecedent(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            UnidirectionalConditionalContentsValidationErrorKind::Consequent(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            UnidirectionalConditionalContentsValidationErrorKind::OpenerFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::fmt::Display for UnidirectionalConditionalPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            UnidirectionalConditionalPositionsValidationErrorKind::Antecedent(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            UnidirectionalConditionalPositionsValidationErrorKind::Consequent(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            UnidirectionalConditionalPositionsValidationErrorKind::Opener(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            UnidirectionalConditionalPositionsValidationErrorKind::OpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for UnidirectionalConditionalContentsValidationError {}

impl std::error::Error for UnidirectionalConditionalPositionsValidationError {}

impl<StringType, TokenStringType: From<&'static str>> Tokenize<TokenStringType>
    for UnidirectionalConditional<StringType>
where
    Block<StringType>: Tokenize<TokenStringType>,
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens = self
            .opener_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        tokens.push(Token {
            content: TokenContent::Identifier(TokenStringType::from(
                CONDITIONAL_ANTECEDENT_OPENER,
            )),
            position: self.opener_position,
        });
        tokens.extend(self.antecedent.tokenize());
        tokens.extend(self.consequent.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_unidirectional_conditional_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<UnidirectionalConditional<$source_string_type>>
            for UnidirectionalConditional<$target_string_type>
        {
            fn from(
                value: UnidirectionalConditional<$source_string_type>,
            ) -> Self {
                UnidirectionalConditional {
                    antecedent: Box::new((*value.antecedent).into()),
                    consequent: value.consequent.into(),
                    opener_position: value.opener_position,
                    opener_fillers: value
                        .opener_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_unidirectional_conditional_string_type_conversion!(&str, Arc<str>);
impl_unidirectional_conditional_string_type_conversion!(&str, Box<str>);
impl_unidirectional_conditional_string_type_conversion!(&str, Rc<str>);
impl_unidirectional_conditional_string_type_conversion!(&str, String);
impl_unidirectional_conditional_string_type_conversion!(Box<str>, Arc<str>);
impl_unidirectional_conditional_string_type_conversion!(Box<str>, Rc<str>);
impl_unidirectional_conditional_string_type_conversion!(Box<str>, String);
impl_unidirectional_conditional_string_type_conversion!(String, Arc<str>);
impl_unidirectional_conditional_string_type_conversion!(String, Box<str>);
impl_unidirectional_conditional_string_type_conversion!(String, Rc<str>);
