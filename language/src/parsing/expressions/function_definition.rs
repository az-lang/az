use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::keywords::FUNCTION_DEFINITION_OPENER;
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::annotated_identifier::{
    AnnotatedIdentifier, AnnotatedIdentifierContentsValidationError,
    AnnotatedIdentifierPositionsValidationError,
};
use super::block::{
    Block, BlockContentsValidationError, BlockPositionsValidationError,
};
use super::expression::{
    Expression, ExpressionContentsValidationError,
    ExpressionPositionsValidationError,
};
use super::utils::range_to_containment_string;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDefinition<StringType> {
    pub parameters: Vec<AnnotatedIdentifier<StringType>>,
    pub return_type: Box<Expression<StringType>>,
    pub body: Block<StringType>,
    pub opener_position: SubstringPosition,
    pub open_parenthesis_position: SubstringPosition,
    pub comma_positions: Vec<SubstringPosition>,
    pub close_parenthesis_position: SubstringPosition,
    pub arrow_position: SubstringPosition,
    pub opener_fillers: FillerVec<StringType>,
    pub open_parenthesis_fillers: FillerVec<StringType>,
    pub comma_fillers: Vec<FillerVec<StringType>>,
    pub close_parenthesis_fillers: FillerVec<StringType>,
    pub arrow_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> FunctionDefinition<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<FunctionDefinitionContentsValidationError>> {
        let mut errors = vec![];
        if let Err(opener_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.opener_fillers)
        {
            errors.extend(opener_filler_errors.into_iter().map(|error| {
                FunctionDefinitionContentsValidationError(
                    FunctionDefinitionContentsValidationErrorKind::OpenerFiller(
                        error,
                    ),
                )
            }));
        }
        if let Err(open_parenthesis_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.open_parenthesis_fillers)
        {
            errors.extend(open_parenthesis_filler_errors.into_iter().map(
                |error| FunctionDefinitionContentsValidationError(
                    FunctionDefinitionContentsValidationErrorKind::OpenParenthesisFiller(error)
                ),
            ));
        }
        if self.comma_positions.len() != self.comma_fillers.len() {
            errors.push(FunctionDefinitionContentsValidationError(
                FunctionDefinitionContentsValidationErrorKind::CommaPositionsNumberCommaFillersNumberMismatch {
                fillers_number: self.comma_fillers.len(),
                positions_number: self.comma_positions.len(),
            }
            ));
        } else if !function_definition_parameters_count_to_comma_count_range(
            self.parameters.len(),
        )
        .contains(&self.comma_positions.len())
        {
            errors.push(FunctionDefinitionContentsValidationError(
                FunctionDefinitionContentsValidationErrorKind::CommasNumberParametersNumberMismatch {
                    commas_number: self.comma_positions.len(),
                    parameters_number: self.parameters.len(),
                },
            ));
        };
        for (parameter, comma_fillers) in
            self.parameters.iter().zip(&self.comma_fillers)
        {
            if let Err(parameter_errors) = parameter.validate_contents_impl() {
                errors.extend(parameter_errors.into_iter().map(
                        |error| FunctionDefinitionContentsValidationError(
                            FunctionDefinitionContentsValidationErrorKind::Parameter(error)
                        ),
                    ));
            }
            if let Err(comma_filler_errors) = validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(comma_fillers)
            {
                errors.extend(comma_filler_errors.into_iter().map(
                        |error| {
                            FunctionDefinitionContentsValidationError(
                                FunctionDefinitionContentsValidationErrorKind::CommaFiller(error)
                            )
                        },
                    ));
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
                        FunctionDefinitionContentsValidationError(
                            FunctionDefinitionContentsValidationErrorKind::Parameter(error)
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
                    FunctionDefinitionContentsValidationError(
                        FunctionDefinitionContentsValidationErrorKind::CloseParenthesisFiller(
                            error,
                        ),
                    )
                },
            ));
        }
        if let Err(arrow_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.arrow_fillers)
        {
            errors.extend(arrow_filler_errors.into_iter().map(|error| {
                FunctionDefinitionContentsValidationError(
                    FunctionDefinitionContentsValidationErrorKind::ArrowFiller(
                        error,
                    ),
                )
            }));
        }
        if let Err(return_type_errors) =
            self.return_type.validate_contents_impl()
        {
            errors.extend(
                return_type_errors.into_iter().map(Box::new).map(
                    |error| FunctionDefinitionContentsValidationError(
                        FunctionDefinitionContentsValidationErrorKind::ReturnType(error)
                    ),
                ),
            );
        }
        if let Err(body_errors) = self.body.validate_contents_impl() {
            errors.extend(body_errors.into_iter().map(|error| {
                FunctionDefinitionContentsValidationError(
                    FunctionDefinitionContentsValidationErrorKind::Body(error),
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
    FunctionDefinition<StringType>
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
    ) -> PositionsValidationStep<FunctionDefinitionPositionsValidationError>
    {
        let mut result = validate_filler_vec_positions(
            &self.opener_fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            FunctionDefinitionPositionsValidationError(
                FunctionDefinitionPositionsValidationErrorKind::OpenerFiller(
                    error,
                ),
            )
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                FUNCTION_DEFINITION_OPENER,
                &self.opener_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                FunctionDefinitionPositionsValidationError(
                    FunctionDefinitionPositionsValidationErrorKind::Opener(
                        error,
                    ),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            validate_filler_vec_positions(
                &self.open_parenthesis_fillers,
                expected_start_character_position,
            )
            .map_error(|error| {
                FunctionDefinitionPositionsValidationError(
                    FunctionDefinitionPositionsValidationErrorKind::OpenParenthesisFiller(
                        error,
                    ),
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
                FunctionDefinitionPositionsValidationError(
                    FunctionDefinitionPositionsValidationErrorKind::OpenParenthesis(
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
                            FunctionDefinitionPositionsValidationError(
                                FunctionDefinitionPositionsValidationErrorKind::Parameter(
                                    error,
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
                        FunctionDefinitionPositionsValidationError(
                            FunctionDefinitionPositionsValidationErrorKind::CommaFiller(
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
                        FunctionDefinitionPositionsValidationError(
                            FunctionDefinitionPositionsValidationErrorKind::Comma(
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
                            FunctionDefinitionPositionsValidationError(
                                FunctionDefinitionPositionsValidationErrorKind::Parameter(
                                    error
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
                FunctionDefinitionPositionsValidationError(
                    FunctionDefinitionPositionsValidationErrorKind::CloseParenthesisFiller(
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
                FunctionDefinitionPositionsValidationError(
                    FunctionDefinitionPositionsValidationErrorKind::CloseParenthesis(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            validate_filler_vec_positions(
                &self.arrow_fillers,
                expected_start_character_position,
            ).map_error(|error| {
                FunctionDefinitionPositionsValidationError(
                    FunctionDefinitionPositionsValidationErrorKind::ArrowFiller(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &TokenContent::<StringType>::Arrow.to_string(),
                &self.arrow_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                FunctionDefinitionPositionsValidationError(
                    FunctionDefinitionPositionsValidationErrorKind::Arrow(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            self.return_type
                .validate_positions_impl(expected_start_character_position)
                .map_error(|error| {
                    FunctionDefinitionPositionsValidationError(
                        FunctionDefinitionPositionsValidationErrorKind::ReturnType(
                            Box::new(error),
                        ),
                    )
                })
        })
        .merge_with(|expected_start_character_position| {
            self.body
                .validate_positions_impl(expected_start_character_position)
                .map_error(|error| {
                    FunctionDefinitionPositionsValidationError(
                        FunctionDefinitionPositionsValidationErrorKind::Body(error),
                    )
                })
        })
    }
}

impl<StringType> FunctionDefinition<StringType> {
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
pub(super) struct FunctionDefinitionContentsValidationError(
    FunctionDefinitionContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct FunctionDefinitionPositionsValidationError(
    FunctionDefinitionPositionsValidationErrorKind,
);

#[derive(Debug)]
enum FunctionDefinitionContentsValidationErrorKind {
    ArrowFiller(FillerVecContentsValidationError),
    Body(BlockContentsValidationError),
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
    OpenerFiller(FillerVecContentsValidationError),
    Parameter(AnnotatedIdentifierContentsValidationError),
    ReturnType(Box<ExpressionContentsValidationError>),
}

#[derive(Debug)]
enum FunctionDefinitionPositionsValidationErrorKind {
    Arrow(PositionsValidationError),
    ArrowFiller(FillerVecPositionsValidationError),
    Body(BlockPositionsValidationError),
    CloseParenthesis(PositionsValidationError),
    CloseParenthesisFiller(FillerVecPositionsValidationError),
    Comma(PositionsValidationError),
    CommaFiller(FillerVecPositionsValidationError),
    OpenParenthesis(PositionsValidationError),
    OpenParenthesisFiller(FillerVecPositionsValidationError),
    Opener(PositionsValidationError),
    OpenerFiller(FillerVecPositionsValidationError),
    Parameter(AnnotatedIdentifierPositionsValidationError),
    ReturnType(Box<ExpressionPositionsValidationError>),
}

const fn function_definition_parameters_count_to_comma_count_range(
    value: usize,
) -> std::ops::RangeInclusive<usize> {
    value.saturating_sub(1usize)..=value
}

impl std::fmt::Display for FunctionDefinitionContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            FunctionDefinitionContentsValidationErrorKind::ArrowFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionContentsValidationErrorKind::Body(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionContentsValidationErrorKind::CloseParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionContentsValidationErrorKind::CommaFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionContentsValidationErrorKind::CommasNumberParametersNumberMismatch {
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
            FunctionDefinitionContentsValidationErrorKind::CommaPositionsNumberCommaFillersNumberMismatch {
                fillers_number,
                positions_number
            } => {
                write!(
                    formatter,
                    "number of commas' fillers should be equal to number of commas' positions, but found {} fillers and {} positions",
                    fillers_number, positions_number
                )
            }
            FunctionDefinitionContentsValidationErrorKind::OpenParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionContentsValidationErrorKind::OpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionContentsValidationErrorKind::Parameter(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionContentsValidationErrorKind::ReturnType(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for FunctionDefinitionPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            FunctionDefinitionPositionsValidationErrorKind::Arrow(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::ArrowFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::Body(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::CloseParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::CloseParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::Comma(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::CommaFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::OpenParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::OpenParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::Opener(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::OpenerFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::Parameter(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            FunctionDefinitionPositionsValidationErrorKind::ReturnType(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for FunctionDefinitionContentsValidationError {}

impl std::error::Error for FunctionDefinitionPositionsValidationError {}

impl<StringType, TokenStringType: From<&'static str>> Tokenize<TokenStringType>
    for FunctionDefinition<StringType>
where
    AnnotatedIdentifier<StringType>: Tokenize<TokenStringType>,
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
                FUNCTION_DEFINITION_OPENER,
            )),
            position: self.opener_position,
        });
        tokens
            .extend(self.open_parenthesis_fillers.into_iter().map(Into::into));
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
        tokens.extend(self.arrow_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: TokenContent::Arrow,
            position: self.arrow_position,
        });
        tokens.extend(self.return_type.tokenize());
        tokens.extend(self.body.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_function_definition_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<FunctionDefinition<$source_string_type>>
            for FunctionDefinition<$target_string_type>
        {
            fn from(value: FunctionDefinition<$source_string_type>) -> Self {
                FunctionDefinition {
                    parameters: value
                        .parameters
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                    return_type: Box::new((*value.return_type).into()),
                    body: value.body.into(),
                    opener_position: value.opener_position,
                    open_parenthesis_position: value.open_parenthesis_position,
                    comma_positions: value.comma_positions,
                    close_parenthesis_position: value
                        .close_parenthesis_position,
                    arrow_position: value.arrow_position,
                    opener_fillers: value
                        .opener_fillers
                        .into_iter()
                        .map(Into::into)
                        .collect(),
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
                    arrow_fillers: value
                        .arrow_fillers
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
