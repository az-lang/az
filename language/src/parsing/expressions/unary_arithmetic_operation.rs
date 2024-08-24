use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::operators::UnaryArithmeticOperator;
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::tokenization::{
    ByteSize, CharacterPosition, PositionsValidationError, SubstringPosition,
    Token, TokenCollection, TokenContent, Tokenize, Utf8Size,
};

use super::expression::{Expression, ExpressionPositionsValidationError};
use super::unary_operand::{
    validate_maybe_nested_unary_operand, UnaryOperandMetadata,
    UnaryOperandValidationError,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct UnaryArithmeticOperation<StringType> {
    pub operand: Box<Expression<StringType>>,
    pub operator: UnaryArithmeticOperator,
    pub operator_position: SubstringPosition,
    pub operator_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> UnaryArithmeticOperation<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<UnaryArithmeticOperationContentsValidationError>> {
        let mut errors = vec![];
        if let Err(operator_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.operator_fillers)
        {
            errors.extend(
                operator_filler_errors.into_iter().map(
                    |error| UnaryArithmeticOperationContentsValidationError(
                        UnaryArithmeticOperationContentsValidationErrorKind::OperatorFiller(error)
                    ),
                ),
            );
        }
        errors.extend(
            validate_maybe_nested_unary_operand(
                self.operand.as_ref(),
                self.operator,
            )
            .into_iter()
            .map(|error| {
                UnaryArithmeticOperationContentsValidationError(
                    UnaryArithmeticOperationContentsValidationErrorKind::Operand(
                        error,
                    ),
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
    UnaryArithmeticOperation<StringType>
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
        UnaryArithmeticOperationPositionsValidationError,
    > {
        validate_filler_vec_positions(
            &self.operator_fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            UnaryArithmeticOperationPositionsValidationError(
                UnaryArithmeticOperationPositionsValidationErrorKind::OperatorFiller(
                    error,
                ),
            )
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &TokenContent::<StringType>::from(self.operator).to_string(),
                &self.operator_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                UnaryArithmeticOperationPositionsValidationError(
                    UnaryArithmeticOperationPositionsValidationErrorKind::Operator(
                        error,
                    ),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            self.operand
                .validate_positions_impl(
                    expected_start_character_position,
                )
                .map_error(|error| {
                    UnaryArithmeticOperationPositionsValidationError(
                        UnaryArithmeticOperationPositionsValidationErrorKind::Operand(
                            Box::new(error),
                        ),
                    )
                })
        })
    }
}

impl<StringType> UnaryArithmeticOperation<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.operator_fillers
            .first()
            .map(|filler| filler.position.start)
            .unwrap_or(self.operator_position.start)
    }
}

#[derive(Debug)]
pub(super) struct UnaryArithmeticOperationContentsValidationError(
    UnaryArithmeticOperationContentsValidationErrorKind,
);

#[derive(Debug)]
enum UnaryArithmeticOperationContentsValidationErrorKind {
    Operand(UnaryOperandValidationError<UnaryArithmeticOperator>),
    OperatorFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
pub(super) struct UnaryArithmeticOperationPositionsValidationError(
    UnaryArithmeticOperationPositionsValidationErrorKind,
);

#[derive(Debug)]
enum UnaryArithmeticOperationPositionsValidationErrorKind {
    Operand(Box<ExpressionPositionsValidationError>),
    Operator(PositionsValidationError),
    OperatorFiller(FillerVecPositionsValidationError),
}

impl UnaryOperandMetadata for UnaryArithmeticOperator {
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        match self {
            UnaryArithmeticOperator::Negation => "operand",
        }
    }
}

impl std::fmt::Display for UnaryArithmeticOperationContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            UnaryArithmeticOperationContentsValidationErrorKind::Operand(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            UnaryArithmeticOperationContentsValidationErrorKind::OperatorFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::fmt::Display for UnaryArithmeticOperationPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            UnaryArithmeticOperationPositionsValidationErrorKind::Operand(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            UnaryArithmeticOperationPositionsValidationErrorKind::Operator(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
            UnaryArithmeticOperationPositionsValidationErrorKind::OperatorFiller(
                error,
            ) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for UnaryArithmeticOperationContentsValidationError {}

impl std::error::Error for UnaryArithmeticOperationPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for UnaryArithmeticOperation<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens = self
            .operator_fillers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        tokens.push(Token {
            content: self.operator.into(),
            position: self.operator_position,
        });
        tokens.extend(self.operand.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_unary_arithmetic_operation_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<UnaryArithmeticOperation<$source_string_type>>
            for UnaryArithmeticOperation<$target_string_type>
        {
            fn from(
                value: UnaryArithmeticOperation<$source_string_type>,
            ) -> Self {
                UnaryArithmeticOperation {
                    operand: Box::new((*value.operand).into()),
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

impl_unary_arithmetic_operation_string_type_conversion!(&str, Arc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(&str, Box<str>);
impl_unary_arithmetic_operation_string_type_conversion!(&str, Rc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(&str, String);
impl_unary_arithmetic_operation_string_type_conversion!(Box<str>, Arc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(Box<str>, Rc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(Box<str>, String);
impl_unary_arithmetic_operation_string_type_conversion!(String, Arc<str>);
impl_unary_arithmetic_operation_string_type_conversion!(String, Box<str>);
impl_unary_arithmetic_operation_string_type_conversion!(String, Rc<str>);
