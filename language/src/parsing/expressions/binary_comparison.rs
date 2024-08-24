use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::operators::BinaryComparisonOperator;
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::binary_operand::{
    validate_maybe_nested_left_binary_operand,
    validate_maybe_nested_right_binary_operand,
    BinaryLeftOperandValidationError, BinaryOperandMetadata,
    BinaryRightOperandValidationError, LEFT_BINARY_OPERAND_POSITION,
    RIGHT_BINARY_OPERAND_POSITION,
};
use super::expression::{Expression, ExpressionPositionsValidationError};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryComparison<StringType> {
    pub left: Box<Expression<StringType>>,
    pub right: Box<Expression<StringType>>,
    pub operator: BinaryComparisonOperator,
    pub operator_position: SubstringPosition,
    pub operator_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> BinaryComparison<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<BinaryComparisonContentsValidationError>> {
        let mut errors = vec![];
        errors.extend(
            validate_maybe_nested_left_binary_operand(
                self.left.as_ref(),
                self.operator,
            )
            .into_iter()
            .map(|error| {
                BinaryComparisonContentsValidationError(
                    BinaryComparisonContentsValidationErrorKind::Left(error),
                )
            }),
        );
        if let Err(operator_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.operator_fillers)
        {
            errors.extend(operator_filler_errors.into_iter().map(|error| {
                BinaryComparisonContentsValidationError(
                    BinaryComparisonContentsValidationErrorKind::OperatorFiller(
                        error,
                    ),
                )
            }));
        }
        errors.extend(
            validate_maybe_nested_right_binary_operand(
                self.right.as_ref(),
                self.operator,
            )
            .into_iter()
            .map(|error| {
                BinaryComparisonContentsValidationError(
                    BinaryComparisonContentsValidationErrorKind::Right(error),
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size>
    BinaryComparison<StringType>
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
    ) -> PositionsValidationStep<BinaryComparisonPositionsValidationError>
    {
        self.left
            .validate_positions_impl(expected_start_character_position)
            .map_error(|error| {
                BinaryComparisonPositionsValidationError(
                    BinaryComparisonPositionsValidationErrorKind::Left(
                        Box::new(error),
                    ),
                )
            })
            .merge_with(|expected_start_character_position| {
                validate_filler_vec_positions(
                    &self.operator_fillers,
                    expected_start_character_position,
                )
                    .map_error(|error| {
                        BinaryComparisonPositionsValidationError(
                            BinaryComparisonPositionsValidationErrorKind::OperatorFiller(
                                error,
                            ),
                        )
                    })
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    &TokenContent::<StringType>::from(self.operator).to_string(),
                    &self.operator_position,
                    expected_start_character_position
                ).map_error(
                    |error| {
                        BinaryComparisonPositionsValidationError(
                            BinaryComparisonPositionsValidationErrorKind::Operator(
                                error,
                            ),
                        )
                    },
                )
            })
            .merge_with(|expected_start_character_position| {
                self.right
                    .validate_positions_impl(
                        expected_start_character_position,
                    )
                    .map_error(|error| {
                        BinaryComparisonPositionsValidationError(
                            BinaryComparisonPositionsValidationErrorKind::Right(
                                Box::new(error),
                            ),
                        )
                    })
            })
    }
}

impl<StringType> BinaryComparison<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.left.to_first_start_character_position()
    }
}

#[derive(Debug)]
pub(super) struct BinaryComparisonContentsValidationError(
    BinaryComparisonContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct BinaryComparisonPositionsValidationError(
    BinaryComparisonPositionsValidationErrorKind,
);

#[derive(Debug)]
enum BinaryComparisonContentsValidationErrorKind {
    Left(BinaryLeftOperandValidationError<BinaryComparisonOperator>),
    OperatorFiller(FillerVecContentsValidationError),
    Right(BinaryRightOperandValidationError<BinaryComparisonOperator>),
}

#[derive(Debug)]
enum BinaryComparisonPositionsValidationErrorKind {
    Left(Box<ExpressionPositionsValidationError>),
    Operator(PositionsValidationError),
    OperatorFiller(FillerVecPositionsValidationError),
    Right(Box<ExpressionPositionsValidationError>),
}

impl BinaryOperandMetadata<LEFT_BINARY_OPERAND_POSITION>
    for BinaryComparisonOperator
{
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        match self {
            BinaryComparisonOperator::EqualTo
            | BinaryComparisonOperator::GreaterThan
            | BinaryComparisonOperator::GreaterThanOrEqualTo
            | BinaryComparisonOperator::LessThan
            | BinaryComparisonOperator::LessThanOrEqualTo
            | BinaryComparisonOperator::NotEqualTo => "left operand",
        }
    }
}

impl BinaryOperandMetadata<RIGHT_BINARY_OPERAND_POSITION>
    for BinaryComparisonOperator
{
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        match self {
            BinaryComparisonOperator::EqualTo
            | BinaryComparisonOperator::GreaterThan
            | BinaryComparisonOperator::GreaterThanOrEqualTo
            | BinaryComparisonOperator::LessThan
            | BinaryComparisonOperator::LessThanOrEqualTo
            | BinaryComparisonOperator::NotEqualTo => "right operand",
        }
    }
}

impl std::fmt::Display for BinaryComparisonContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            BinaryComparisonContentsValidationErrorKind::Left(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BinaryComparisonContentsValidationErrorKind::OperatorFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            BinaryComparisonContentsValidationErrorKind::Right(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for BinaryComparisonPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            BinaryComparisonPositionsValidationErrorKind::Left(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BinaryComparisonPositionsValidationErrorKind::Operator(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BinaryComparisonPositionsValidationErrorKind::OperatorFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            BinaryComparisonPositionsValidationErrorKind::Right(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for BinaryComparisonContentsValidationError {}

impl std::error::Error for BinaryComparisonPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for BinaryComparison<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens = self.left.tokenize().into_iter().collect::<Vec<_>>();
        tokens.extend(self.operator_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: self.operator.into(),
            position: self.operator_position,
        });
        tokens.extend(self.right.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_comparison_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<BinaryComparison<$source_string_type>>
            for BinaryComparison<$target_string_type>
        {
            fn from(value: BinaryComparison<$source_string_type>) -> Self {
                BinaryComparison {
                    left: Box::new((*value.left).into()),
                    right: Box::new((*value.right).into()),
                    operator: value.operator,
                    operator_position: value.operator_position,
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

impl_comparison_string_type_conversion!(&str, Arc<str>);
impl_comparison_string_type_conversion!(&str, Box<str>);
impl_comparison_string_type_conversion!(&str, Rc<str>);
impl_comparison_string_type_conversion!(&str, String);
impl_comparison_string_type_conversion!(Box<str>, Arc<str>);
impl_comparison_string_type_conversion!(Box<str>, Rc<str>);
impl_comparison_string_type_conversion!(Box<str>, String);
impl_comparison_string_type_conversion!(String, Arc<str>);
impl_comparison_string_type_conversion!(String, Box<str>);
impl_comparison_string_type_conversion!(String, Rc<str>);
