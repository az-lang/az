use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::expression::{
    Expression, ExpressionContentsValidationError,
    ExpressionPositionsValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Grouping<StringType> {
    pub expression: Box<Expression<StringType>>,
    pub open_parenthesis_position: SubstringPosition,
    pub close_parenthesis_position: SubstringPosition,
    pub open_parenthesis_fillers: FillerVec<StringType>,
    pub close_parenthesis_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> Grouping<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<GroupingContentsValidationError>> {
        let mut errors = vec![];
        if let Err(open_parenthesis_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.open_parenthesis_fillers)
        {
            errors.extend(open_parenthesis_filler_errors.into_iter().map(
                |error| {
                    GroupingContentsValidationError(
                        GroupingContentsValidationErrorKind::OpenParenthesisFiller(error),
                    )
                },
            ));
        }
        if let Err(expression_errors) =
            self.expression.validate_contents_impl()
        {
            errors.extend(expression_errors.into_iter().map(|error| {
                GroupingContentsValidationError(
                    GroupingContentsValidationErrorKind::Expression(Box::new(
                        error,
                    )),
                )
            }));
        }
        if let Err(close_parenthesis_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.close_parenthesis_fillers)
        {
            errors.extend(close_parenthesis_filler_errors.into_iter().map(
                |error| {
                    GroupingContentsValidationError(
                        GroupingContentsValidationErrorKind::CloseParenthesisFiller(error),
                    )
                },
            ));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Grouping<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<GroupingPositionsValidationError> {
        validate_filler_vec_positions(
            &self.open_parenthesis_fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            GroupingPositionsValidationError(
                GroupingPositionsValidationErrorKind::OpenParenthesisFiller(
                    error,
                ),
            )
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &TokenContent::<StringType>::OpenParenthesis.to_string(),
                &self.open_parenthesis_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                GroupingPositionsValidationError(
                    GroupingPositionsValidationErrorKind::OpenParenthesis(
                        error,
                    ),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            self.expression
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    GroupingPositionsValidationError(
                        GroupingPositionsValidationErrorKind::Expression(
                            Box::new(error),
                        ),
                    )
                })
        })
        .merge_with(|expected_start_character_position| {
            validate_filler_vec_positions(
                &self.close_parenthesis_fillers,
                expected_start_character_position,
            )
            .map_error(|error| {
                GroupingPositionsValidationError(
                    GroupingPositionsValidationErrorKind::CloseParenthesisFiller(
                        error,
                    ),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &TokenContent::<StringType>::CloseParenthesis.to_string(),
                &self.close_parenthesis_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                GroupingPositionsValidationError(
                    GroupingPositionsValidationErrorKind::CloseParenthesis(
                        error,
                    ),
                )
            })
        })
    }
}

impl<StringType> Grouping<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.open_parenthesis_fillers
            .first()
            .map(|filler| filler.position.start)
            .unwrap_or(self.open_parenthesis_position.start)
    }
}

#[derive(Debug)]
pub(super) struct GroupingContentsValidationError(
    GroupingContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct GroupingPositionsValidationError(
    GroupingPositionsValidationErrorKind,
);

#[derive(Debug)]
enum GroupingContentsValidationErrorKind {
    CloseParenthesisFiller(FillerVecContentsValidationError),
    Expression(Box<ExpressionContentsValidationError>),
    OpenParenthesisFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
enum GroupingPositionsValidationErrorKind {
    CloseParenthesis(PositionsValidationError),
    CloseParenthesisFiller(FillerVecPositionsValidationError),
    Expression(Box<ExpressionPositionsValidationError>),
    OpenParenthesis(PositionsValidationError),
    OpenParenthesisFiller(FillerVecPositionsValidationError),
}

impl std::fmt::Display for GroupingContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            GroupingContentsValidationErrorKind::CloseParenthesisFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            GroupingContentsValidationErrorKind::Expression(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            GroupingContentsValidationErrorKind::OpenParenthesisFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::fmt::Display for GroupingPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            GroupingPositionsValidationErrorKind::CloseParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            GroupingPositionsValidationErrorKind::CloseParenthesisFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            GroupingPositionsValidationErrorKind::Expression(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            GroupingPositionsValidationErrorKind::OpenParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            GroupingPositionsValidationErrorKind::OpenParenthesisFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for GroupingContentsValidationError {}

impl std::error::Error for GroupingPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Grouping<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens = self
            .open_parenthesis_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        tokens.push(Token {
            content: TokenContent::OpenParenthesis,
            position: self.open_parenthesis_position,
        });
        tokens.extend(self.expression.tokenize());
        tokens.extend(
            self.close_parenthesis_fillers.into_iter().map(Into::into),
        );
        tokens.push(Token {
            content: TokenContent::CloseParenthesis,
            position: self.close_parenthesis_position,
        });
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_grouping_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Grouping<$source_string_type>>
            for Grouping<$target_string_type>
        {
            fn from(value: Grouping<$source_string_type>) -> Self {
                Grouping {
                    expression: Box::new((*value.expression).into()),
                    open_parenthesis_position: value.open_parenthesis_position,
                    close_parenthesis_position: value
                        .close_parenthesis_position,
                    open_parenthesis_fillers: value
                        .open_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    close_parenthesis_fillers: value
                        .close_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_grouping_string_type_conversion!(&str, Arc<str>);
impl_grouping_string_type_conversion!(&str, Box<str>);
impl_grouping_string_type_conversion!(&str, Rc<str>);
impl_grouping_string_type_conversion!(&str, String);
impl_grouping_string_type_conversion!(Box<str>, Arc<str>);
impl_grouping_string_type_conversion!(Box<str>, Rc<str>);
impl_grouping_string_type_conversion!(Box<str>, String);
impl_grouping_string_type_conversion!(String, Arc<str>);
impl_grouping_string_type_conversion!(String, Box<str>);
impl_grouping_string_type_conversion!(String, Rc<str>);
