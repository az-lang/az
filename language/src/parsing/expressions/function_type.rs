use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::expressions::binary_operand::{
    validate_maybe_nested_right_binary_operand, BinaryOperandMetadata,
    BinaryRightOperandValidationError, RIGHT_BINARY_OPERAND_POSITION,
};
use crate::parsing::expressions::ExpressionPositionsValidationError;
use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::operators::FunctionTypeOperator;
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::expression::{Expression, ExpressionContentsValidationError};
use super::utils::range_to_containment_string;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType<StringType> {
    pub parameters: Vec<Expression<StringType>>,
    pub return_type: Box<Expression<StringType>>,
    pub open_parenthesis_position: SubstringPosition,
    pub comma_positions: Vec<SubstringPosition>,
    pub close_parenthesis_position: SubstringPosition,
    pub operator_position: SubstringPosition,
    pub open_parenthesis_fillers: FillerVec<StringType>,
    pub comma_fillers: Vec<FillerVec<StringType>>,
    pub close_parenthesis_fillers: FillerVec<StringType>,
    pub operator_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> FunctionType<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<FunctionTypeContentsValidationError>> {
        let mut errors = vec![];
        if let Err(open_parenthesis_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.open_parenthesis_fillers)
        {
            errors.extend(open_parenthesis_filler_errors.into_iter().map(
                |error| FunctionTypeContentsValidationError(
                    FunctionTypeContentsValidationErrorKind::OpenParenthesisFiller(error)
                ),
            ));
        }
        if self.comma_positions.len() != self.comma_fillers.len() {
            errors.push(FunctionTypeContentsValidationError(
                FunctionTypeContentsValidationErrorKind::CommaPositionsNumberCommaFillersNumberMismatch {
                fillers_number: self.comma_fillers.len(),
                positions_number: self.comma_positions.len(),
            }
            ));
        } else if !function_definition_parameters_count_to_comma_count_range(
            self.parameters.len(),
        )
        .contains(&self.comma_positions.len())
        {
            errors.push(FunctionTypeContentsValidationError(
                FunctionTypeContentsValidationErrorKind::CommasNumberParametersNumberMismatch {
                    commas_number: self.comma_positions.len(),
                    parameters_number: self.parameters.len(),
                },
            ));
        };
        for (parameter, comma_fillers) in
            self.parameters.iter().zip(&self.comma_fillers)
        {
            if let Err(parameter_errors) = parameter.validate_contents_impl() {
                errors.extend(parameter_errors.into_iter().map(|error| {
                    FunctionTypeContentsValidationError(
                        FunctionTypeContentsValidationErrorKind::Parameter(
                            Box::new(error),
                        ),
                    )
                }));
            }
            if let Err(comma_filler_errors) = validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(comma_fillers)
            {
                errors.extend(comma_filler_errors.into_iter().map(|error| {
                    FunctionTypeContentsValidationError(
                        FunctionTypeContentsValidationErrorKind::CommaFiller(
                            error,
                        ),
                    )
                }));
            }
        }
        if self.parameters.len() > self.comma_positions.len() {
            let last_parameter =
                unsafe { self.parameters.last().unwrap_unchecked() };
            if let Err(last_parameter_errors) =
                last_parameter.validate_contents_impl()
            {
                errors.extend(last_parameter_errors.into_iter().map(
                    |error| {
                        FunctionTypeContentsValidationError(
                            FunctionTypeContentsValidationErrorKind::Parameter(
                                Box::new(error),
                            ),
                        )
                    },
                ));
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
                    FunctionTypeContentsValidationError(
                        FunctionTypeContentsValidationErrorKind::CloseParenthesisFiller(
                            error,
                        ),
                    )
                },
            ));
        }
        if let Err(arrow_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.operator_fillers)
        {
            errors.extend(arrow_filler_errors.into_iter().map(|error| {
                FunctionTypeContentsValidationError(
                    FunctionTypeContentsValidationErrorKind::ArrowFiller(
                        error,
                    ),
                )
            }));
        }
        errors.extend(
            validate_maybe_nested_right_binary_operand(
                self.return_type.as_ref(),
                FunctionTypeOperator,
            )
            .into_iter()
            .map(|error| {
                FunctionTypeContentsValidationError(
                    FunctionTypeContentsValidationErrorKind::ReturnType(error),
                )
            }),
        );
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<StringType: AsRef<str> + ByteSize + Utf8Size> FunctionType<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<FunctionTypePositionsValidationError> {
        let mut result = validate_filler_vec_positions(
            &self.open_parenthesis_fillers,
            expected_start_character_position,
        )
            .map_error(|error| {
                FunctionTypePositionsValidationError(
                    FunctionTypePositionsValidationErrorKind::OpenParenthesisFiller(
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
                    FunctionTypePositionsValidationError(
                        FunctionTypePositionsValidationErrorKind::OpenParenthesis(
                            error,
                        ),
                    )
                })
            });
        for ((parameter, comma_fillers), comma_position) in self
            .parameters
            .iter()
            .zip(&self.comma_fillers)
            .zip(&self.comma_positions)
        {
            result = result
                .merge_with(|expected_start_character_position| {
                    parameter
                        .validate_positions_impl(
                            expected_start_character_position,
                        )
                        .map_error(|error| {
                            FunctionTypePositionsValidationError(
                                FunctionTypePositionsValidationErrorKind::Parameter(
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
                        FunctionTypePositionsValidationError(
                            FunctionTypePositionsValidationErrorKind::CommaFiller(
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
                        FunctionTypePositionsValidationError(
                            FunctionTypePositionsValidationErrorKind::Comma(
                                error,
                            ),
                        )
                    })
                });
        }
        if self.parameters.len() > self.comma_positions.len() {
            result =
                result.merge_with(|expected_start_character_position| {
                    unsafe { self.parameters.last().unwrap_unchecked() }
                        .validate_positions_impl(
                            expected_start_character_position,
                        )
                        .map_error(|error| {
                            FunctionTypePositionsValidationError(
                                FunctionTypePositionsValidationErrorKind::Parameter(
                                    Box::new(error),
                                ),
                            )
                        })
                });
        }
        result.merge_with(|expected_start_character_position| {
            validate_filler_vec_positions(
                &self.close_parenthesis_fillers,
                expected_start_character_position,
            )
            .map_error(|error| {
                FunctionTypePositionsValidationError(
                    FunctionTypePositionsValidationErrorKind::CloseParenthesisFiller(
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
                FunctionTypePositionsValidationError(
                    FunctionTypePositionsValidationErrorKind::CloseParenthesis(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            validate_filler_vec_positions(
                &self.operator_fillers,
                expected_start_character_position,
            ).map_error(|error| {
                FunctionTypePositionsValidationError(
                    FunctionTypePositionsValidationErrorKind::OperatorFiller(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &TokenContent::<StringType>::Arrow.to_string(),
                &self.operator_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                FunctionTypePositionsValidationError(
                    FunctionTypePositionsValidationErrorKind::Operator(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            self.return_type
                .validate_positions_impl(expected_start_character_position)
                .map_error(|error| {
                    FunctionTypePositionsValidationError(
                        FunctionTypePositionsValidationErrorKind::ReturnType(
                            Box::new(error),
                        ),
                    )
                })
        })
    }
}

impl<StringType> FunctionType<StringType> {
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
pub(super) struct FunctionTypeContentsValidationError(
    FunctionTypeContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct FunctionTypePositionsValidationError(
    FunctionTypePositionsValidationErrorKind,
);

#[derive(Debug)]
enum FunctionTypeContentsValidationErrorKind {
    ArrowFiller(FillerVecContentsValidationError),
    CloseParenthesisFiller(FillerVecContentsValidationError),
    CommaFiller(FillerVecContentsValidationError),
    CommasNumberParametersNumberMismatch {
        commas_number: usize,
        parameters_number: usize,
    },
    CommaPositionsNumberCommaFillersNumberMismatch {
        fillers_number: usize,
        positions_number: usize,
    },
    OpenParenthesisFiller(FillerVecContentsValidationError),
    Parameter(Box<ExpressionContentsValidationError>),
    ReturnType(BinaryRightOperandValidationError<FunctionTypeOperator>),
}

#[derive(Debug)]
enum FunctionTypePositionsValidationErrorKind {
    CloseParenthesis(PositionsValidationError),
    CloseParenthesisFiller(FillerVecPositionsValidationError),
    Comma(PositionsValidationError),
    CommaFiller(FillerVecPositionsValidationError),
    OpenParenthesis(PositionsValidationError),
    OpenParenthesisFiller(FillerVecPositionsValidationError),
    Operator(PositionsValidationError),
    OperatorFiller(FillerVecPositionsValidationError),
    Parameter(Box<ExpressionPositionsValidationError>),
    ReturnType(Box<ExpressionPositionsValidationError>),
}

impl BinaryOperandMetadata<RIGHT_BINARY_OPERAND_POSITION>
    for FunctionTypeOperator
{
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        "return type"
    }
}

const fn function_definition_parameters_count_to_comma_count_range(
    value: usize,
) -> std::ops::RangeInclusive<usize> {
    value.saturating_sub(1usize)..=value
}

impl std::fmt::Display for FunctionTypeContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            FunctionTypeContentsValidationErrorKind::ArrowFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypeContentsValidationErrorKind::CloseParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypeContentsValidationErrorKind::CommaFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypeContentsValidationErrorKind::CommasNumberParametersNumberMismatch {
                commas_number,
                parameters_number
            } => write!(
                formatter,
                "number of commas should be {}, but found {}",
                range_to_containment_string(&function_definition_parameters_count_to_comma_count_range(
                    *parameters_number
                )),
                commas_number
            ),
            FunctionTypeContentsValidationErrorKind::CommaPositionsNumberCommaFillersNumberMismatch {
                fillers_number,
                positions_number
            } => {
                write!(
                    formatter,
                    "number of commas' fillers should be equal to number of commas' positions, but found {} fillers and {} positions",
                    fillers_number, positions_number
                )
            }
            FunctionTypeContentsValidationErrorKind::OpenParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypeContentsValidationErrorKind::Parameter(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypeContentsValidationErrorKind::ReturnType(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for FunctionTypePositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            FunctionTypePositionsValidationErrorKind::Operator(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::OperatorFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::CloseParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::CloseParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::Comma(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::CommaFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::OpenParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::OpenParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::Parameter(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionTypePositionsValidationErrorKind::ReturnType(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for FunctionTypeContentsValidationError {}

impl std::error::Error for FunctionTypePositionsValidationError {}

impl<StringType, TokenStringType: From<&'static str>> Tokenize<TokenStringType>
    for FunctionType<StringType>
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
        if self.parameters.len() > self.comma_positions.len() {
            let mut parameters = self.parameters;
            let last_parameter =
                unsafe { parameters.pop().unwrap_unchecked() };
            parameters
                .into_iter()
                .zip(self.comma_fillers.into_iter().zip(self.comma_positions))
                .for_each(|(parameter, (comma_fillers, comma_position))| {
                    tokens.extend(parameter.tokenize());
                    tokens.extend(comma_fillers.into_iter().map(Into::into));
                    tokens.push(Token {
                        content: TokenContent::Comma,
                        position: comma_position,
                    });
                });
            tokens.extend(last_parameter.tokenize());
        } else {
            self.parameters
                .into_iter()
                .zip(self.comma_fillers.into_iter().zip(self.comma_positions))
                .for_each(|(parameter, (comma_fillers, comma_position))| {
                    tokens.extend(parameter.tokenize());
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
        tokens.extend(self.operator_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: TokenContent::Arrow,
            position: self.operator_position,
        });
        tokens.extend(self.return_type.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_function_definition_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<FunctionType<$source_string_type>>
            for FunctionType<$target_string_type>
        {
            fn from(value: FunctionType<$source_string_type>) -> Self {
                FunctionType {
                    parameters: value
                        .parameters
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    return_type: Box::new((*value.return_type).into()),
                    open_parenthesis_position: value.open_parenthesis_position,
                    comma_positions: value.comma_positions,
                    close_parenthesis_position: value
                        .close_parenthesis_position,
                    operator_position: value.operator_position,
                    open_parenthesis_fillers: value
                        .open_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    comma_fillers: value
                        .comma_fillers
                        .into_iter()
                        .map(|comma_fillers| {
                            comma_fillers.into_iter().map(Into::into).collect()
                        })
                        .collect(),
                    close_parenthesis_fillers: value
                        .close_parenthesis_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    operator_fillers: value
                        .operator_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            }
        }
    };
}

impl_function_definition_string_type_conversion!(&str, Arc<str>);
impl_function_definition_string_type_conversion!(&str, Box<str>);
impl_function_definition_string_type_conversion!(&str, Rc<str>);
impl_function_definition_string_type_conversion!(&str, String);
impl_function_definition_string_type_conversion!(Box<str>, Arc<str>);
impl_function_definition_string_type_conversion!(Box<str>, Rc<str>);
impl_function_definition_string_type_conversion!(Box<str>, String);
impl_function_definition_string_type_conversion!(String, Arc<str>);
impl_function_definition_string_type_conversion!(String, Box<str>);
impl_function_definition_string_type_conversion!(String, Rc<str>);
