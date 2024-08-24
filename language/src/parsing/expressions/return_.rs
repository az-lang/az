use std::rc::Rc;
use std::sync::Arc;

use crate::parsing::filler::{
    validate_filler_vec_contents, validate_filler_vec_positions, Filler,
    FillerVec, FillerVecContentsValidationError,
    FillerVecPositionsValidationError, NON_TERMINAL_FILLER_VEC_POSITION,
};
use crate::parsing::positions_validation_step::PositionsValidationStep;
use crate::parsing::ReturnOperator;
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
pub struct Return<StringType> {
    pub expression: Box<Expression<StringType>>,
    pub operator_position: SubstringPosition,
    pub operator_fillers: FillerVec<StringType>,
}

impl<StringType: AsRef<str>> Return<StringType>
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
    ) -> Result<(), Vec<ReturnContentsValidationError>> {
        let mut errors = vec![];
        if let Err(operator_filler_errors) = validate_filler_vec_contents::<
            NON_TERMINAL_FILLER_VEC_POSITION,
            StringType,
        >(&self.operator_fillers)
        {
            errors.extend(operator_filler_errors.into_iter().map(|error| {
                ReturnContentsValidationError(
                    ReturnContentsValidationErrorKind::OperatorFiller(error),
                )
            }));
        }
        errors.extend(
            validate_maybe_nested_unary_operand(
                self.expression.as_ref(),
                ReturnOperator,
            )
            .into_iter()
            .map(|error| {
                ReturnContentsValidationError(
                    ReturnContentsValidationErrorKind::Expression(error),
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

impl<StringType: AsRef<str> + ByteSize + Utf8Size> Return<StringType> {
    pub fn validate_positions(
        &self,
    ) -> Result<(), Vec<impl std::error::Error>> {
        self.validate_positions_impl(self.to_first_start_character_position())
            .into_result()
    }

    pub(super) fn validate_positions_impl(
        &self,
        expected_start_character_position: CharacterPosition,
    ) -> PositionsValidationStep<ReturnPositionsValidationError> {
        validate_filler_vec_positions(
            &self.operator_fillers,
            expected_start_character_position,
        )
        .map_error(|error| {
            ReturnPositionsValidationError(
                ReturnPositionsValidationErrorKind::OperatorFiller(error),
            )
        })
        .merge_with(|expected_start_character_position| {
            PositionsValidationStep::from_string(
                &ReturnOperator.into_const_token_content().to_string(),
                &self.operator_position,
                expected_start_character_position,
            )
            .map_error(|error| {
                ReturnPositionsValidationError(
                    ReturnPositionsValidationErrorKind::Operator(error),
                )
            })
        })
        .merge_with(|expected_start_character_position| {
            self.expression
                .validate_positions_impl(expected_start_character_position)
                .map_error(|error| {
                    ReturnPositionsValidationError(
                        ReturnPositionsValidationErrorKind::Expression(
                            Box::new(error),
                        ),
                    )
                })
        })
    }
}

impl<StringType> Return<StringType> {
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
pub(super) struct ReturnContentsValidationError(
    ReturnContentsValidationErrorKind,
);

#[derive(Debug)]
pub(super) struct ReturnPositionsValidationError(
    ReturnPositionsValidationErrorKind,
);

#[derive(Debug)]
enum ReturnContentsValidationErrorKind {
    Expression(UnaryOperandValidationError<ReturnOperator>),
    OperatorFiller(FillerVecContentsValidationError),
}

#[derive(Debug)]
enum ReturnPositionsValidationErrorKind {
    Expression(Box<ExpressionPositionsValidationError>),
    Operator(PositionsValidationError),
    OperatorFiller(FillerVecPositionsValidationError),
}

impl UnaryOperandMetadata for ReturnOperator {
    type Name = &'static str;

    fn to_operand_name(&self) -> Self::Name {
        "expression"
    }
}

impl std::fmt::Display for ReturnContentsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            ReturnContentsValidationErrorKind::Expression(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ReturnContentsValidationErrorKind::OperatorFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::fmt::Display for ReturnPositionsValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match &self.0 {
            ReturnPositionsValidationErrorKind::Expression(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ReturnPositionsValidationErrorKind::Operator(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            ReturnPositionsValidationErrorKind::OperatorFiller(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl std::error::Error for ReturnContentsValidationError {}

impl std::error::Error for ReturnPositionsValidationError {}

impl<StringType, TokenStringType: From<&'static str>> Tokenize<TokenStringType>
    for Return<StringType>
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
            content: TokenContent::from(ReturnOperator),
            position: self.operator_position,
        });
        tokens.extend(self.expression.tokenize());
        TokenCollection::new(tokens)
    }
}

macro_rules! impl_return_string_type_conversion {
    ($source_string_type:ty, $target_string_type:ty) => {
        impl From<Return<$source_string_type>>
            for Return<$target_string_type>
        {
            fn from(value: Return<$source_string_type>) -> Self {
                Return {
                    expression: Box::new((*value.expression).into()),
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

impl_return_string_type_conversion!(&str, Arc<str>);
impl_return_string_type_conversion!(&str, Box<str>);
impl_return_string_type_conversion!(&str, Rc<str>);
impl_return_string_type_conversion!(&str, String);
impl_return_string_type_conversion!(Box<str>, Arc<str>);
impl_return_string_type_conversion!(Box<str>, Rc<str>);
impl_return_string_type_conversion!(Box<str>, String);
impl_return_string_type_conversion!(String, Arc<str>);
impl_return_string_type_conversion!(String, Box<str>);
impl_return_string_type_conversion!(String, Rc<str>);
