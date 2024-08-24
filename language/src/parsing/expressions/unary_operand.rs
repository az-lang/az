use std::cmp::Ordering;

use crate::parsing::precedence::Precedence;
use crate::tokenization::{TokenCollection, TokenContent};

use super::expression::{Expression, ExpressionContentsValidationError};
use super::to_reduced_first_token_content::ToReducedFirstTokenContent;

pub(super) trait UnaryOperandMetadata {
    type Name;

    fn to_operand_name(&self) -> Self::Name;
}

pub(super) fn validate_maybe_nested_unary_operand<
    UnaryOperator: Copy,
    StringType: AsRef<str>,
>(
    operand: &Expression<StringType>,
    operator: UnaryOperator,
) -> Vec<UnaryOperandValidationError<UnaryOperator>>
where
    Precedence: From<UnaryOperator>,
    TokenContent<&'static str>:
        std::fmt::Display + From<UnaryOperator> + PartialEq,
{
    let mut result =
        validate_maybe_nested_unary_operand_order(operand, operator)
            .into_iter()
            .map(|error| {
                UnaryOperandValidationError(
                    UnaryOperandValidationErrorKind::Order(error),
                )
            })
            .collect::<Vec<_>>();
    {
        let operator_token_content = TokenContent::from(operator);
        let operator_conflicts_with_operand = match TokenCollection::try_from(
            format!(
                "{}{}",
                operator_token_content,
                operand.to_reduced_first_token_content()
            )
            .as_str(),
        ) {
            Ok(tokens) => !tokens
                .into_iter()
                .next()
                .is_some_and(|token| token.content == operator_token_content),
            Err(_) => true,
        };
        if operator_conflicts_with_operand {
            result.push(UnaryOperandValidationError(
                UnaryOperandValidationErrorKind::OperatorLexicalConflict(
                    operator,
                ),
            ));
        }
    }
    if let Err(operand_errors) = operand.validate_contents_impl() {
        result.extend(operand_errors.into_iter().map(|error| {
            UnaryOperandValidationError(
                UnaryOperandValidationErrorKind::Contents(Box::new(error)),
            )
        }));
    }
    result
}

fn validate_maybe_nested_unary_operand_order<UnaryOperator: Copy, StringType>(
    operand: &Expression<StringType>,
    operator: UnaryOperator,
) -> Vec<UnaryOperandOrderValidationError<UnaryOperator>>
where
    Precedence: From<UnaryOperator>,
{
    let mut result = vec![];
    if let Some(operand_precedence) = operand.to_operation_precedence() {
        match operand_precedence.cmp(&Precedence::from(operator)) {
            Ordering::Less => {
                result.push(
                    UnaryOperandOrderValidationError::OperandHasLesserPrecedenceThanOperator(operator)
                );
            }
            Ordering::Equal | Ordering::Greater => {}
        }
    };
    result
}

#[derive(Debug)]
pub(super) struct UnaryOperandValidationError<UnaryOperator>(
    UnaryOperandValidationErrorKind<UnaryOperator>,
);

#[derive(Debug)]
enum UnaryOperandOrderValidationError<UnaryOperator> {
    OperandHasLesserPrecedenceThanOperator(UnaryOperator),
}

#[derive(Debug)]
enum UnaryOperandValidationErrorKind<UnaryOperator> {
    Contents(Box<ExpressionContentsValidationError>),
    Order(UnaryOperandOrderValidationError<UnaryOperator>),
    OperatorLexicalConflict(UnaryOperator),
}

impl<UnaryOperator: Copy + UnaryOperandMetadata> std::fmt::Display
    for UnaryOperandOrderValidationError<UnaryOperator>
where
    <UnaryOperator as UnaryOperandMetadata>::Name: std::fmt::Display,
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::OperandHasLesserPrecedenceThanOperator(operator) => {
                write!(
                    formatter,
                    "{} has lesser precedence than the operator",
                    operator.to_operand_name()
                )
            }
        }
    }
}

impl<UnaryOperator> std::fmt::Display
    for UnaryOperandValidationError<UnaryOperator>
where
    UnaryOperandValidationErrorKind<UnaryOperator>: std::fmt::Display,
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl<UnaryOperator: UnaryOperandMetadata> std::fmt::Display
    for UnaryOperandValidationErrorKind<UnaryOperator>
where
    <UnaryOperator as UnaryOperandMetadata>::Name: std::fmt::Display,
    UnaryOperandOrderValidationError<UnaryOperator>: std::fmt::Display,
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Contents(error) => std::fmt::Display::fmt(error, formatter),
            Self::Order(error) => std::fmt::Display::fmt(error, formatter),
            Self::OperatorLexicalConflict(operator) => {
                write!(
                    formatter,
                    "{} lexically conflicts with the operator",
                    operator.to_operand_name()
                )
            }
        }
    }
}
