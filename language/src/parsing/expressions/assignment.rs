use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::operators::AssignmentOperator;
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
pub struct Assignment<StringType> {
    pub target: Box<Expression<StringType>>,
    pub value: Box<Expression<StringType>>,
    pub operator_position: SubstringPosition,
    pub operator_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> Assignment<StringType> {
    pub fn validate_contents(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_contents_impl()
    }

    pub(super) fn validate_contents_impl(
        &self,
    ) -> Result<(), Vec<AssignmentContentsValidationError>> {
        let mut errors = vec![];
        errors.extend(
            validate_maybe_nested_left_binary_operand(
                self.target.as_ref(),
                AssignmentOperator,
            )
            .into_iter()
            .map(|error| {
                AssignmentContentsValidationError(
                    AssignmentContentsValidationErrorKind::Target(error),
                )
            }),
        );
        if let Err(operator_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.operator_fillers)
        {
            errors.extend(operator_filler_errors.into_iter().map(|error| {
                AssignmentContentsValidationError(
                    AssignmentContentsValidationErrorKind::OperatorFiller(
                        error,
                    ),
                )
            }));
        }
        errors.extend(
            validate_maybe_nested_right_binary_operand(
                self.target.as_ref(),
                AssignmentOperator,
            )
            .into_iter()
            .map(|error| {
                AssignmentContentsValidationError(
                    AssignmentContentsValidationErrorKind::Value(error),
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Assignment<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<AssignmentPositionsValidationError> {
        self.target
            .validate_positions_impl(expected_start_character_position)
            .map_error(|error| {
                AssignmentPositionsValidationError(
                    AssignmentPositionsValidationErrorKind::Target(Box::new(
                        error,
                    )),
                )
            })
            .merge_with(|expected_start_character_position| {
                validate_filler_vec_positions(
                    &self.operator_fillers,
                    expected_start_character_position,
                )
                .map_error(|error| {
                    AssignmentPositionsValidationError(
                        AssignmentPositionsValidationErrorKind::OperatorFiller(
                            error,
                        ),
                    )
                })
            })
            .merge_with(|expected_start_character_position| {
                PositionsValidationStep::from_string(
                    &TokenContent::<StringType>::from(AssignmentOperator)
                        .to_string(),
                    &self.operator_position,
                    expected_start_character_position,
                )
                .map_error(|error| {
                    AssignmentPositionsValidationError(
                        AssignmentPositionsValidationErrorKind::Operator(
                            error,
                        ),
                    )
                })
            })
            .merge_with(|expected_start_character_position| {
                self.value
                    .validate_positions_impl(expected_start_character_position)
                    .map_error(|error| {
                        AssignmentPositionsValidationError(
                            AssignmentPositionsValidationErrorKind::Value(
                                Box::new(error),
                            ),
                        )
                    })
            })
    }
}

impl<StringType> Assignment<StringType> {
    pub(super) fn to_first_start_character_position(
        &self,
    ) -> CharacterPosition {
        self.target.to_first_start_character_position()
    }
}

#[derive(Debug)]
pub(super) struct AssignmentContentsValidationError(
    AssignmentContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct AssignmentPositionsValidationError(
    AssignmentPositionsValidationErrorKind,
);

#[derive(Debug)]
enum AssignmentContentsValidationErrorKind {
    OperatorFiller(FillerVecContentsValidationError),
    Target(BinaryLeftOperandValidationError<AssignmentOperator>),
    Value(BinaryRightOperandValidationError<AssignmentOperator>),
}

#[derive(Debug)]
enum AssignmentPositionsValidationErrorKind {
    Operator(PositionsValidationError),
    OperatorFiller(FillerVecPositionsValidationError),
    Target(Box<ExpressionPositionsValidationError>),
    Value(Box<ExpressionPositionsValidationError>),
}

impl BinaryOperandMetadata<LEFT_BINARY_OPERAND_POSITION>
    for AssignmentOperator
{
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        "target"
    }
}

impl BinaryOperandMetadata<RIGHT_BINARY_OPERAND_POSITION>
    for AssignmentOperator
{
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        "value"
    }
}

impl std::fmt::Display for AssignmentContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            AssignmentContentsValidationErrorKind::OperatorFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            AssignmentContentsValidationErrorKind::Target(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            AssignmentContentsValidationErrorKind::Value(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for AssignmentPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            AssignmentPositionsValidationErrorKind::Operator(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            AssignmentPositionsValidationErrorKind::OperatorFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            AssignmentPositionsValidationErrorKind::Target(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            AssignmentPositionsValidationErrorKind::Value(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for AssignmentContentsValidationError {}

impl std::error::Error for AssignmentPositionsValidationError {}

impl<StringType, TokenStringType> Tokenize<TokenStringType>
    for Assignment<StringType>
where
    Expression<StringType>: Tokenize<TokenStringType>,
    Filler<StringType>: Into<Token<TokenStringType>>,
{
    fn tokenize(self) -> TokenCollection<TokenStringType> {
        let mut tokens =
            self.target.tokenize().into_iter().collect::<Vec<_>>();
        tokens.extend(self.operator_fillers.into_iter().map(Into::into));
        tokens.push(Token {
            content: AssignmentOperator.into(),
            position: self.operator_position,
        });
        tokens.extend(self.value.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_assignment_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Assignment<$source_string_type>>
            for Assignment<$target_string_type>
        {
            fn from(value: Assignment<$source_string_type>) -> Self {
                Assignment {
                    target: Box::new((*value.target).into()),
                    value: Box::new((*value.value).into()),
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

impl_assignment_string_type_conversion!(&str, Arc<str>);
impl_assignment_string_type_conversion!(&str, Box<str>);
impl_assignment_string_type_conversion!(&str, Rc<str>);
impl_assignment_string_type_conversion!(&str, String);
impl_assignment_string_type_conversion!(Box<str>, Arc<str>);
impl_assignment_string_type_conversion!(Box<str>, Rc<str>);
impl_assignment_string_type_conversion!(Box<str>, String);
impl_assignment_string_type_conversion!(String, Arc<str>);
impl_assignment_string_type_conversion!(String, Box<str>);
impl_assignment_string_type_conversion!(String, Rc<str>);
