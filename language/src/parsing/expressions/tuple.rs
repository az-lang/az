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
use super::utils::range_to_containment_string;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Tuple<StringType> {
    pub elements: Vec<Expression<StringType>>,
    pub open_parenthesis_position: SubstringPosition,
    pub comma_positions: Vec<SubstringPosition>,
    pub close_parenthesis_position: SubstringPosition,
    pub open_parenthesis_fillers: FillerVec<StringType>,
    pub comma_fillers: Vec<FillerVec<StringType>>,
    pub close_parenthesis_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> Tuple<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<TupleContentsValidationError>> {
        let mut errors = vec![];
        if let Err(open_parenthesis_filler_errors) =
            validate_filler_vec_contents::<
                NON_TERMINAL_FILLER_VEC_POSITION,
                StringType,
            >(&self.open_parenthesis_fillers)
        {
            errors.extend(open_parenthesis_filler_errors.into_iter().map(
                |error| {
                    TupleContentsValidationError(
                        TupleContentsValidationErrorKind::OpenParenthesisFiller(error),
                    )
                },
            ));
        }
        if self.comma_positions.len() != self.comma_fillers.len() {
            errors.push(TupleContentsValidationError(
                TupleContentsValidationErrorKind::CommaPositionsNumberCommaFillersNumberMismatch {
                    fillers_number: self.comma_fillers.len(),
                    positions_number: self.comma_positions.len(),
                }
            ));
        } else if !tuple_elements_count_to_comma_count_range(
            self.elements.len(),
        )
        .contains(&self.comma_positions.len())
        {
            errors.push(TupleContentsValidationError(
                TupleContentsValidationErrorKind::CommasNumberElementsNumberMismatch {
                    commas_number: self.comma_positions.len(),
                    elements_number: self.elements.len(),
                },
            ));
        };
        self.elements.iter().zip(&self.comma_fillers).for_each(
            |(element, comma_fillers)| {
                if let Err(element_errors) = element.validate_contents_impl() {
                    errors.extend(element_errors.into_iter().map(|error| {
                        TupleContentsValidationError(
                            TupleContentsValidationErrorKind::Element(
                                Box::new(error),
                            ),
                        )
                    }));
                }
                if let Err(comma_filler_errors) =
                    validate_filler_vec_contents::<
                        NON_TERMINAL_FILLER_VEC_POSITION,
                        StringType,
                    >(comma_fillers)
                {
                    errors.extend(comma_filler_errors.into_iter().map(
                        |error| {
                            TupleContentsValidationError(
                                TupleContentsValidationErrorKind::CommaFiller(
                                    error,
                                ),
                            )
                        },
                    ));
                }
            },
        );
        if self.elements.len() > self.comma_positions.len() {
            let last_element =
                unsafe { self.elements.last().unwrap_unchecked() };
            if let Err(last_element_errors) =
                last_element.validate_contents_impl()
            {
                errors.extend(last_element_errors.into_iter().map(|error| {
                    TupleContentsValidationError(
                        TupleContentsValidationErrorKind::Element(Box::new(
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
                    TupleContentsValidationError(
                        TupleContentsValidationErrorKind::CloseParenthesisFiller(error),
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Tuple<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<TuplePositionsValidationError> {
        let mut result = validate_filler_vec_positions(
            &self.open_parenthesis_fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            TuplePositionsValidationError(
                TuplePositionsValidationErrorKind::OpenParenthesisFiller(
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
                TuplePositionsValidationError(
                    TuplePositionsValidationErrorKind::OpenParenthesis(error),
                )
            })
        });
        for ((element, comma_fillers), comma_position) in self
            .elements
            .iter()
            .zip(&self.comma_fillers)
            .zip(&self.comma_positions)
        {
            result = result
                .merge_with(|expected_start_character_position| {
                    element
                        .validate_positions_impl(
                            expected_start_character_position,
                        )
                        .map_error(|error| {
                            TuplePositionsValidationError(
                                TuplePositionsValidationErrorKind::Element(
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
                        TuplePositionsValidationError(
                            TuplePositionsValidationErrorKind::CommaFiller(
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
                        TuplePositionsValidationError(
                            TuplePositionsValidationErrorKind::Comma(error),
                        )
                    })
                });
        }
        if self.elements.len() > self.comma_positions.len() {
            result = result.merge_with(|expected_start_character_position| {
                unsafe { self.elements.last().unwrap_unchecked() }
                    .validate_positions_impl(expected_start_character_position)
                    .map_error(|error| {
                        TuplePositionsValidationError(
                            TuplePositionsValidationErrorKind::Element(
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
                    TuplePositionsValidationError(
                    TuplePositionsValidationErrorKind::CloseParenthesisFiller(
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
                    TuplePositionsValidationError(
                        TuplePositionsValidationErrorKind::CloseParenthesis(
                            error,
                        ),
                    )
                })
            })
    }
}

impl<StringType> Tuple<StringType> {
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
pub(super) struct TupleContentsValidationError(
    TupleContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct TuplePositionsValidationError(
    TuplePositionsValidationErrorKind,
);

#[derive(Debug)]
enum TupleContentsValidationErrorKind {
    CloseParenthesisFiller(FillerVecContentsValidationError),
    CommaFiller(FillerVecContentsValidationError),
    CommaPositionsNumberCommaFillersNumberMismatch {
        fillers_number: usize,
        positions_number: usize,
    },
    CommasNumberElementsNumberMismatch {
        commas_number: usize,
        elements_number: usize,
    },
    Element(Box<ExpressionContentsValidationError>),
    OpenParenthesisFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
enum TuplePositionsValidationErrorKind {
    CloseParenthesis(PositionsValidationError),
    CloseParenthesisFiller(FillerVecPositionsValidationError),
    Comma(PositionsValidationError),
    CommaFiller(FillerVecPositionsValidationError),
    Element(Box<ExpressionPositionsValidationError>),
    OpenParenthesis(PositionsValidationError),
    OpenParenthesisFiller(FillerVecPositionsValidationError),
}

impl std::fmt::Display for TupleContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            TupleContentsValidationErrorKind::Element(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TupleContentsValidationErrorKind::CloseParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TupleContentsValidationErrorKind::CommaFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TupleContentsValidationErrorKind::CommaPositionsNumberCommaFillersNumberMismatch {
                fillers_number,
                positions_number
            } => {
                write!(
                    formatter,
                    "number of commas' fillers should be equal to number of commas' positions, but found {} fillers and {} positions",
                    fillers_number, positions_number
                )
            }
            TupleContentsValidationErrorKind::CommasNumberElementsNumberMismatch {
                commas_number,
                elements_number
            } => write!(
                formatter,
                "number of commas should be {}, but found {}",
                range_to_containment_string(&tuple_elements_count_to_comma_count_range(
                    *elements_number
                )),
                commas_number
            ),
            TupleContentsValidationErrorKind::OpenParenthesisFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for TuplePositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            TuplePositionsValidationErrorKind::Element(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TuplePositionsValidationErrorKind::CloseParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TuplePositionsValidationErrorKind::CloseParenthesisFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            TuplePositionsValidationErrorKind::Comma(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TuplePositionsValidationErrorKind::CommaFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TuplePositionsValidationErrorKind::OpenParenthesis(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            TuplePositionsValidationErrorKind::OpenParenthesisFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for TupleContentsValidationError {}

impl std::error::Error for TuplePositionsValidationError {}

const fn tuple_elements_count_to_comma_count_range(
    value: usize,
) -> std::ops::RangeInclusive<usize> {
    (if value <= 1usize {
        value
    } else {
        value - 1usize
    })..=value
}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Tuple<StringType>
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
        if self.elements.len() > self.comma_positions.len() {
            let mut elements = self.elements;
            let last_element = unsafe { elements.pop().unwrap_unchecked() };
            elements
                .into_iter()
                .zip(self.comma_fillers.into_iter().zip(self.comma_positions))
                .for_each(|(element, (comma_fillers, comma_position))| {
                    tokens.extend(element.tokenize());
                    tokens.extend(comma_fillers.into_iter().map(Into::into));
                    tokens.push(Token {
                        content: TokenContent::Comma,
                        position: comma_position,
                    });
                });
            tokens.extend(last_element.tokenize());
        } else {
            self.elements
                .into_iter()
                .zip(self.comma_fillers.into_iter().zip(self.comma_positions))
                .for_each(|(element, (comma_fillers, comma_position))| {
                    tokens.extend(element.tokenize());
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

macro_rules! impl_tuple_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Tuple<$source_string_type>> for Tuple<$target_string_type> {
            fn from(value: Tuple<$source_string_type>) -> Self {
                Tuple {
                    elements: value
                        .elements
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

impl_tuple_string_type_conversion!(&str, Arc<str>);
impl_tuple_string_type_conversion!(&str, Box<str>);
impl_tuple_string_type_conversion!(&str, Rc<str>);
impl_tuple_string_type_conversion!(&str, String);
impl_tuple_string_type_conversion!(Box<str>, Arc<str>);
impl_tuple_string_type_conversion!(Box<str>, Rc<str>);
impl_tuple_string_type_conversion!(Box<str>, String);
impl_tuple_string_type_conversion!(String, Arc<str>);
impl_tuple_string_type_conversion!(String, Box<str>);
impl_tuple_string_type_conversion!(String, Rc<str>);
