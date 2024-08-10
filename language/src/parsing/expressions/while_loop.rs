use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::keywords::WHILE_LOOP_OPENER;
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
pub struct WhileLoop<StringType> {
    pub condition: Box<Expression<StringType>>,
    pub body: Block<StringType>,
    pub opener_position: SubstringPosition,
    pub opener_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> WhileLoop<StringType>
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
    ) -> Result<(), Vec<WhileLoopContentsValidationError>> {
        let mut errors = vec![];
        if let Err(opener_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.opener_fillers)
        {
            errors.extend(opener_filler_errors.into_iter().map(|error| {
                WhileLoopContentsValidationError(
                    WhileLoopContentsValidationErrorKind::OpenerFiller(error),
                )
            }));
        }
        {
            const OPENER_TOKEN_CONTENT: TokenContent<&str> =
                TokenContent::Identifier(WHILE_LOOP_OPENER);
            let opener_conflicts_with_condition =
                match TokenCollection::try_from(
                    format!(
                        "{}{}",
                        OPENER_TOKEN_CONTENT,
                        self.condition
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
            if opener_conflicts_with_condition {
                errors.push(WhileLoopContentsValidationError(
                    WhileLoopContentsValidationErrorKind::Condition(
                        ConditionValidationErrorKind::OpenerLexicalConflict,
                    ),
                ));
            }
            if let Err(condition_errors) =
                self.condition.validate_contents_impl()
            {
                errors.extend(condition_errors.into_iter().map(|error| {
                    WhileLoopContentsValidationError(
                        WhileLoopContentsValidationErrorKind::Condition(
                            ConditionValidationErrorKind::Contents(Box::new(
                                error,
                            )),
                        ),
                    )
                }));
            }
        }
        if let Err(body_errors) = self.body.validate_contents_impl() {
            errors.extend(body_errors.into_iter().map(|error| {
                WhileLoopContentsValidationError(
                    WhileLoopContentsValidationErrorKind::Body(error),
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size> WhileLoop<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<WhileLoopPositionsValidationError> {
        validate_filler_vec_positions(
            &self.opener_fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            WhileLoopPositionsValidationError(
                WhileLoopPositionsValidationErrorKind::OpenerFiller(error),
            )
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                WHILE_LOOP_OPENER,
                &self.opener_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                WhileLoopPositionsValidationError(
                    WhileLoopPositionsValidationErrorKind::Opener(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            self.condition
                .validate_positions_impl(expected_start_character_position)
                .map_error(|error| {
                    WhileLoopPositionsValidationError(
                        WhileLoopPositionsValidationErrorKind::Condition(
                            Box::new(error),
                        ),
                    )
                })
        })
        .merge_with(|expected_start_character_position| {
            self.body
                .validate_positions_impl(expected_start_character_position)
                .map_error(|error| {
                    WhileLoopPositionsValidationError(
                        WhileLoopPositionsValidationErrorKind::Body(error),
                    )
                })
        })
    }
}

impl<StringType> WhileLoop<StringType> {
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
pub(super) struct WhileLoopContentsValidationError(
    WhileLoopContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct WhileLoopPositionsValidationError(
    WhileLoopPositionsValidationErrorKind,
);

#[derive(Debug)]
enum ConditionValidationErrorKind {
    Contents(Box<ExpressionContentsValidationError>),
    OpenerLexicalConflict,
}

#[derive(Debug)]
enum WhileLoopContentsValidationErrorKind {
    Body(BlockContentsValidationError),
    Condition(ConditionValidationErrorKind),
    OpenerFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
enum WhileLoopPositionsValidationErrorKind {
    Body(BlockPositionsValidationError),
    Condition(Box<ExpressionPositionsValidationError>),
    Opener(PositionsValidationError),
    OpenerFiller(FillerVecPositionsValidationError),
}

impl std::fmt::Display for ConditionValidationErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ConditionValidationErrorKind::Contents(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ConditionValidationErrorKind::OpenerLexicalConflict => {
                write!(
                    formatter,
                    "opener lexically conflicts with the condition"
                )
            }
        }
    }
}

impl std::fmt::Display for WhileLoopContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            WhileLoopContentsValidationErrorKind::Body(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            WhileLoopContentsValidationErrorKind::Condition(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            WhileLoopContentsValidationErrorKind::OpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for WhileLoopPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            WhileLoopPositionsValidationErrorKind::Body(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            WhileLoopPositionsValidationErrorKind::Condition(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            WhileLoopPositionsValidationErrorKind::Opener(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            WhileLoopPositionsValidationErrorKind::OpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for WhileLoopContentsValidationError {}

impl std::error::Error for WhileLoopPositionsValidationError {}

impl<StringType, TokenStringType: From<&'static str>> Tokenize<TokenStringType>
    for WhileLoop<StringType>
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
                WHILE_LOOP_OPENER,
            )),
            position: self.opener_position,
        });
        tokens.extend(self.condition.tokenize());
        tokens.extend(self.body.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_unidirectional_conditional_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<WhileLoop<$source_string_type>>
            for WhileLoop<$target_string_type>
        {
            fn from(value: WhileLoop<$source_string_type>) -> Self {
                WhileLoop {
                    condition: Box::new((*value.condition).into()),
                    body: value.body.into(),
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
