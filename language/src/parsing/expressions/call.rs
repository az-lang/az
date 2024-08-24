use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::operators::CallOperator;
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::binary_operand::{
    validate_maybe_nested_left_binary_operand,
    BinaryLeftOperandValidationError, BinaryOperandMetadata,
    LEFT_BINARY_OPERAND_POSITION,
};
use super::expression::{
    Expression, ExpressionContentsValidationError,
    ExpressionPositionsValidationError,
};
use super::utils::range_to_containment_string;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Call<StringType> {
    pub callable: Box<Expression<StringType>>,
    pub arguments: Vec<Expression<StringType>>,
    pub open_parenthesis_position: SubstringPosition,
    pub comma_positions: Vec<SubstringPosition>,
    pub close_parenthesis_position: SubstringPosition,
    pub open_parenthesis_fillers: FillerVec<StringType>,
    pub comma_fillers: Vec<FillerVec<StringType>>,
    pub close_parenthesis_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> Call<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<CallContentsValidationError>> {
        let mut errors = vec![];
        errors.extend(
            validate_maybe_nested_left_binary_operand(
                self.callable.as_ref(),
                CallOperator,
            )
            .into_iter()
            .map(|error| {
                CallContentsValidationError(
                    CallContentsValidationErrorKind::Callable(error),
                )
            }),
        );
        if let Err(open_parenthesis_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.open_parenthesis_fillers)
        {
            errors.extend(open_parenthesis_filler_errors.into_iter().map(
                |error| {
                    CallContentsValidationError(
                        CallContentsValidationErrorKind::OpenParenthesisFiller(
                            error,
                        ),
                    )
                },
            ));
        }
        if self.comma_positions.len() != self.comma_fillers.len() {
            errors.push(CallContentsValidationError(
                CallContentsValidationErrorKind::CommaPositionsNumberCommaFillersNumberMismatch {
                    fillers_number: self.comma_fillers.len(),
                    positions_number: self.comma_positions.len(),
                }
            ));
        } else if !call_arguments_count_to_comma_count_range(
            self.arguments.len(),
        )
        .contains(&self.comma_positions.len())
        {
            errors.push(CallContentsValidationError(
                CallContentsValidationErrorKind::CommasNumberArgumentsNumberMismatch {
                    commas_number: self.comma_positions.len(),
                    arguments_number: self.arguments.len(),
                },
            ));
        };
        for (argument, comma_fillers) in
            self.arguments.iter().zip(&self.comma_fillers)
        {
            if let Err(argument_errors) = argument.validate_contents_impl() {
                errors.extend(argument_errors.into_iter().map(|error| {
                    CallContentsValidationError(
                        CallContentsValidationErrorKind::Argument(Box::new(
                            error,
                        )),
                    )
                }));
            }
            if let Err(filler_errors) = validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(comma_fillers)
            {
                errors.extend(filler_errors.into_iter().map(|error| {
                    CallContentsValidationError(
                        CallContentsValidationErrorKind::CommaFiller(error),
                    )
                }));
            }
        }
        if self.arguments.len() > self.comma_positions.len() {
            let last_argument =
                unsafe { self.arguments.last().unwrap_unchecked() };
            if let Err(last_argument_errors) =
                last_argument.validate_contents_impl()
            {
                errors.extend(last_argument_errors.into_iter().map(|error| {
                    CallContentsValidationError(
                        CallContentsValidationErrorKind::Argument(Box::new(
                            error,
                        )),
                    )
                }));
            }
        }
        if let Err(close_parenthesis_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.close_parenthesis_fillers)
        {
            errors.extend(close_parenthesis_filler_errors.into_iter().map(
                |error| {
                    CallContentsValidationError(
                        CallContentsValidationErrorKind::CloseParenthesisFiller(
                            error,
                        ),
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Call<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<CallPositionsValidationError> {
        let mut result = self.callable
            .validate_positions_impl(expected_start_character_position)
            .map_error(|error| {
                CallPositionsValidationError(
                    CallPositionsValidationErrorKind::Callable(Box::new(error)),
                )
            })
            .merge_with(|expected_start_character_position| {
                validate_filler_vec_positions(
                    &self.open_parenthesis_fillers,
                    expected_start_character_position,
                )
                    .map_error(|error| {
                        CallPositionsValidationError(
                            CallPositionsValidationErrorKind::OpenParenthesisFiller(error),
                        )
                    })
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    &TokenContent::<StringType>::OpenParenthesis.to_string(),
                    &self.open_parenthesis_position,
                    expected_start_character_position,
                )
                .map_error(|error| {
                    CallPositionsValidationError(
                        CallPositionsValidationErrorKind::OpenParenthesis(
                            error,
                        ),
                    )
                })
            });
        for ((argument, comma_fillers), comma_position) in self
            .arguments
            .iter()
            .zip(&self.comma_fillers)
            .zip(&self.comma_positions)
        {
            result = result
                .merge_with(|expected_start_character_position| {
                    argument
                        .validate_positions_impl(
                            expected_start_character_position,
                        )
                        .map_error(|error| {
                            CallPositionsValidationError(
                                CallPositionsValidationErrorKind::Argument(
                                    Box::new(error),
                                ),
                            )
                        })
                })
                .merge_with(|expected_start_character_position| {
                    validate_filler_vec_positions(
                        comma_fillers,
                        expected_start_character_position,
                    )
                    .map_error(|error| {
                        CallPositionsValidationError(
                            CallPositionsValidationErrorKind::CommaFiller(
                                error,
                            ),
                        )
                    })
                })
                .merge_with(|expected_start_character_position| {
                    PositionsValidationStep::from_string(
                        &TokenContent::<StringType>::Comma.to_string(),
                        comma_position,
                        expected_start_character_position,
                    )
                    .map_error(|error| {
                        CallPositionsValidationError(
                            CallPositionsValidationErrorKind::Comma(error),
                        )
                    })
                });
        }
        if self.arguments.len() > self.comma_positions.len() {
            result = result.merge_with(|expected_start_character_position| {
                unsafe { self.arguments.last().unwrap_unchecked() }
                    .validate_positions_impl(expected_start_character_position)
                    .map_error(|error| {
                        CallPositionsValidationError(
                            CallPositionsValidationErrorKind::Argument(
                                Box::new(error),
                            ),
                        )
                    })
            });
        }
        result
            .merge_with(|expected_start_character_position| {
                validate_filler_vec_positions(
                    &self.close_parenthesis_fillers,
                    expected_start_character_position,
                )
                    .map_error(|error| {
                        CallPositionsValidationError(
                            CallPositionsValidationErrorKind::CloseParenthesisFiller(
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
                    CallPositionsValidationError(
                        CallPositionsValidationErrorKind::CloseParenthesis(
                            error,
                        ),
                    )
                })
            })
    }
}

impl<StringType> Call<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.callable.to_first_start_character_position()
    }
}

#[derive(Debug)]
pub(super) struct CallContentsValidationError(CallContentsValidationErrorKind);

#[derive(Debug)]
pub(super) struct CallPositionsValidationError(
    CallPositionsValidationErrorKind,
);

#[derive(Debug)]
enum CallContentsValidationErrorKind {
    Argument(Box<ExpressionContentsValidationError>),
    Callable(BinaryLeftOperandValidationError<CallOperator>),
    CloseParenthesisFiller(FillerVecContentsValidationError),
    CommaFiller(FillerVecContentsValidationError),
    CommaPositionsNumberCommaFillersNumberMismatch {
        fillers_number: usize,
        positions_number: usize,
    },
    CommasNumberArgumentsNumberMismatch {
        commas_number: usize,
        arguments_number: usize,
    },
    OpenParenthesisFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
enum CallPositionsValidationErrorKind {
    Argument(Box<ExpressionPositionsValidationError>),
    Callable(Box<ExpressionPositionsValidationError>),
    CloseParenthesis(PositionsValidationError),
    CloseParenthesisFiller(FillerVecPositionsValidationError),
    Comma(PositionsValidationError),
    CommaFiller(FillerVecPositionsValidationError),
    OpenParenthesis(PositionsValidationError),
    OpenParenthesisFiller(FillerVecPositionsValidationError),
}

const fn call_arguments_count_to_comma_count_range(
    value: usize,
) -> std::ops::RangeInclusive<usize> {
    value.saturating_sub(1usize)..=value
}

impl BinaryOperandMetadata<LEFT_BINARY_OPERAND_POSITION> for CallOperator {
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        "callable"
    }
}

impl std::fmt::Display for CallContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            CallContentsValidationErrorKind::Argument(error) => {
                std::fmt::Display::fmt(error, formatter)
            },
            CallContentsValidationErrorKind::Callable(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            CallContentsValidationErrorKind::CloseParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            },
            CallContentsValidationErrorKind::CommaFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            },
            CallContentsValidationErrorKind::CommaPositionsNumberCommaFillersNumberMismatch { fillers_number, positions_number } => {
                write!(
                    formatter,
                    "number of commas' fillers should be equal to number of commas' positions, but found {} fillers and {} positions",
                    fillers_number, positions_number
                )
            }
            CallContentsValidationErrorKind::CommasNumberArgumentsNumberMismatch {
                commas_number,
                arguments_number,
            } => write!(
                formatter,
                "number of commas should be {}, but found {}",
                range_to_containment_string(&call_arguments_count_to_comma_count_range(
                    *arguments_number
                )),
                commas_number
            ),
            CallContentsValidationErrorKind::OpenParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            },
        }
    }
}

impl std::fmt::Display for CallPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            CallPositionsValidationErrorKind::Argument(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            CallPositionsValidationErrorKind::Callable(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            CallPositionsValidationErrorKind::CloseParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            CallPositionsValidationErrorKind::CloseParenthesisFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            CallPositionsValidationErrorKind::Comma(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            CallPositionsValidationErrorKind::CommaFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            CallPositionsValidationErrorKind::OpenParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            CallPositionsValidationErrorKind::OpenParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for CallContentsValidationError {}

impl std::error::Error for CallPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Call<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens =
            self.callable.tokenize().into_iter().collect::<Vec<_>>();
        tokens
            .extend(self.open_parenthesis_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: TokenContent::OpenParenthesis,
            position: self.open_parenthesis_position,
        });
        if self.arguments.len() > self.comma_positions.len() {
            let mut arguments = self.arguments;
            let last_argument = unsafe { arguments.pop().unwrap_unchecked() };
            arguments
                .into_iter()
                .zip(self.comma_fillers.into_iter().zip(self.comma_positions))
                .for_each(|(argument, (comma_fillers, comma_position))| {
                    tokens.extend(argument.tokenize());
                    tokens.extend(comma_fillers.into_iter().map(Into::into));
                    tokens.push(Token {
                        content: TokenContent::Comma,
                        position: comma_position,
                    });
                });
            tokens.extend(last_argument.tokenize());
        } else {
            self.arguments
                .into_iter()
                .zip(self.comma_fillers.into_iter().zip(self.comma_positions))
                .for_each(|(argument, (comma_fillers, comma_position))| {
                    tokens.extend(argument.tokenize());
                    tokens.extend(comma_fillers.into_iter().map(Into::into));
                    tokens.push(Token {
                        content: TokenContent::Comma,
                        position: comma_position,
                    });
                });
        }
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

macro_rules! impl_call_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Call<$source_string_type>> for Call<$target_string_type> {
            fn from(value: Call<$source_string_type>) -> Self {
                Call {
                    callable: Box::new((*value.callable).into()),
                    arguments: value
                        .arguments
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    open_parenthesis_position: value.open_parenthesis_position,
                    comma_positions: value.comma_positions,
                    close_parenthesis_position: value
                        .close_parenthesis_position,
                    open_parenthesis_fillers: value
                        .open_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    comma_fillers: value
                        .comma_fillers
                        .into_iter()
                        .map(|fillers| {
                            fillers.into_iter().map(Into::into).collect()
                        })
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

impl_call_string_type_conversion!(&str, Arc<str>);
impl_call_string_type_conversion!(&str, Box<str>);
impl_call_string_type_conversion!(&str, Rc<str>);
impl_call_string_type_conversion!(&str, String);
impl_call_string_type_conversion!(Box<str>, Arc<str>);
impl_call_string_type_conversion!(Box<str>, Rc<str>);
impl_call_string_type_conversion!(Box<str>, String);
impl_call_string_type_conversion!(String, Arc<str>);
impl_call_string_type_conversion!(String, Box<str>);
impl_call_string_type_conversion!(String, Rc<str>);
