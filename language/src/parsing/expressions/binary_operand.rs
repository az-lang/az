use std::cmp::Ordering;

use crate::parsing::associativity::Associativity;
use crate::parsing::precedence::Precedence;
use crate::tokenization::{TokenCollection, TokenContent};

use super::expression::{Expression, ExpressionContentsValidationError};
use super::to_reduced_first_token_content::ToReducedFirstTokenContent;

type BinaryOperandPosition = bool;
pub(super) const LEFT_BINARY_OPERAND_POSITION: BinaryOperandPosition = false;
pub(super) const RIGHT_BINARY_OPERAND_POSITION: BinaryOperandPosition = true;

pub(super) trait BinaryOperandMetadata<
    const OPERAND_POSITION: BinaryOperandPosition,
>
{
    type Name;

    fn to_operand_name(&self) -> Self::Name;
}

pub(super) fn validate_maybe_nested_left_binary_operand<
    BinaryOperator: Copy,
    StringType: AsRef<str>,
>(
    operand: &Expression<StringType>,
    operator: BinaryOperator,
) -> Vec<BinaryLeftOperandValidationError<BinaryOperator>>
where
    Associativity: From<BinaryOperator>,
    Precedence: From<BinaryOperator>,
{
    let mut result =
        validate_maybe_nested_binary_operand_order(operand, operator)
            .into_iter()
            .map(|error| {
                BinaryLeftOperandValidationError(
                    BinaryLeftOperandValidationErrorKind::Order(error),
                )
            })
            .collect::<Vec<_>>();
    if let Err(operand_errors) = operand.validate_contents_impl() {
        result.extend(operand_errors.into_iter().map(|error| {
            BinaryLeftOperandValidationError(
                BinaryLeftOperandValidationErrorKind::Contents(Box::new(
                    error,
                )),
            )
        }));
    }
    result
}

pub(super) fn validate_maybe_nested_right_binary_operand<
    BinaryOperator: Copy,
    StringType: AsRef<str>,
>(
    operand: &Expression<StringType>,
    operator: BinaryOperator,
) -> Vec<BinaryRightOperandValidationError<BinaryOperator>>
where
    Associativity: From<BinaryOperator>,
    Precedence: From<BinaryOperator>,
    for<'a> TokenContent<&'a str>:
        std::fmt::Display + From<BinaryOperator> + PartialEq,
{
    let mut result =
        validate_maybe_nested_binary_operand_order(operand, operator)
            .into_iter()
            .map(|error| {
                BinaryRightOperandValidationError(
                    BinaryRightOperandValidationErrorKind::Order(error),
                )
            })
            .collect::<Vec<_>>();
    {
        let operator_token_content = TokenContent::from(operator);
        let operator_conflicts_with_right_operand =
            match TokenCollection::try_from(
                format!(
                    "{}{}",
                    operator_token_content,
                    operand.to_reduced_first_token_content()
                )
                .as_str(),
            ) {
                Ok(tokens) => {
                    !tokens.into_iter().next().is_some_and(|token| {
                        token.content == operator_token_content
                    })
                }
                Err(_) => true,
            };
        if operator_conflicts_with_right_operand {
            result.push(BinaryRightOperandValidationError(
                BinaryRightOperandValidationErrorKind::OperatorLexicalConflict(
                    operator,
                ),
            ));
        }
    }
    if let Err(operand_errors) = operand.validate_contents_impl() {
        result.extend(operand_errors.into_iter().map(|error| {
            BinaryRightOperandValidationError(
                BinaryRightOperandValidationErrorKind::Contents(Box::new(
                    error,
                )),
            )
        }));
    }
    result
}

fn validate_maybe_nested_binary_operand_order<
    BinaryOperator: Copy,
    StringType,
    const OPERAND_POSITION: BinaryOperandPosition,
>(
    operand: &Expression<StringType>,
    operator: BinaryOperator,
) -> Vec<BinaryOperandOrderValidationError<BinaryOperator, OPERAND_POSITION>>
where
    Associativity: From<BinaryOperator>,
    Precedence: From<BinaryOperator>,
{
    let mut result = vec![];
    if let Some(operand_precedence) = operand.to_operation_precedence() {
        match operand_precedence.cmp(&Precedence::from(operator)) {
            Ordering::Less => {
                result.push(
                    BinaryOperandOrderValidationError::OperandHasLesserPrecedenceThanOperator(operator)
                );
            }
            Ordering::Equal => {
                let operand_associativity = unsafe {
                    operand
                        .to_binary_operation_associativity()
                        .unwrap_unchecked()
                };
                let operator_associativity = Associativity::from(operator);
                if operand_associativity == operator_associativity
                    && operator_associativity
                        == match OPERAND_POSITION {
                            LEFT_BINARY_OPERAND_POSITION => {
                                Associativity::RightToLeft
                            }
                            RIGHT_BINARY_OPERAND_POSITION => {
                                Associativity::LeftToRight
                            }
                        }
                {
                    result.push(
                        BinaryOperandOrderValidationError::OperandHasSameOrderButConflictingAssociativity(operator)
                    );
                }
            }
            Ordering::Greater => {}
        }
    };
    result
}

#[derive(Debug)]
pub(super) struct BinaryLeftOperandValidationError<BinaryOperator>(
    BinaryLeftOperandValidationErrorKind<BinaryOperator>,
);

#[derive(Debug)]
pub(super) struct BinaryRightOperandValidationError<BinaryOperator>(
    BinaryRightOperandValidationErrorKind<BinaryOperator>,
);

#[derive(Debug)]
enum BinaryOperandOrderValidationError<
    BinaryOperator,
    const OPERAND_POSITION: BinaryOperandPosition,
> {
    OperandHasLesserPrecedenceThanOperator(BinaryOperator),
    OperandHasSameOrderButConflictingAssociativity(BinaryOperator),
}

#[derive(Debug)]
enum BinaryLeftOperandValidationErrorKind<BinaryOperator> {
    Contents(Box<ExpressionContentsValidationError>),
    Order(
        BinaryOperandOrderValidationError<
            BinaryOperator,
            LEFT_BINARY_OPERAND_POSITION,
        >,
    ),
}

#[derive(Debug)]
enum BinaryRightOperandValidationErrorKind<BinaryOperator> {
    Contents(Box<ExpressionContentsValidationError>),
    Order(
        BinaryOperandOrderValidationError<
            BinaryOperator,
            RIGHT_BINARY_OPERAND_POSITION,
        >,
    ),
    OperatorLexicalConflict(BinaryOperator),
}

impl<
        BinaryOperator: Copy + BinaryOperandMetadata<OPERAND_POSITION>,
        const OPERAND_POSITION: BinaryOperandPosition,
    > std::fmt::Display
    for BinaryOperandOrderValidationError<BinaryOperator, OPERAND_POSITION>
where
    <BinaryOperator as BinaryOperandMetadata<OPERAND_POSITION>>::Name:
        std::fmt::Display,
    Associativity: From<BinaryOperator>,
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
            Self::OperandHasSameOrderButConflictingAssociativity(operator) => {
                write!(
                    formatter,
                    "{} has the same order as the operator, but due to {} associativity grouping should be different",
                    operator.to_operand_name(),
                    match Associativity::from(*operator) {
                        Associativity::LeftToRight => {
                            "left-to-right"
                        }
                        Associativity::RightToLeft => {
                            "right-to-left"
                        }
                    }
                )
            }
        }
    }
}

impl<BinaryOperator> std::fmt::Display
    for BinaryLeftOperandValidationError<BinaryOperator>
where
    BinaryLeftOperandValidationErrorKind<BinaryOperator>: std::fmt::Display,
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl<BinaryOperator> std::fmt::Display
    for BinaryRightOperandValidationError<BinaryOperator>
where
    BinaryRightOperandValidationErrorKind<BinaryOperator>: std::fmt::Display,
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl<BinaryOperator> std::fmt::Display
    for BinaryLeftOperandValidationErrorKind<BinaryOperator>
where
    BinaryOperandOrderValidationError<
        BinaryOperator,
        LEFT_BINARY_OPERAND_POSITION,
    >: std::fmt::Display,
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            BinaryLeftOperandValidationErrorKind::Contents(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BinaryLeftOperandValidationErrorKind::Order(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
        }
    }
}

impl<BinaryOperator: BinaryOperandMetadata<RIGHT_BINARY_OPERAND_POSITION>>
    std::fmt::Display for BinaryRightOperandValidationErrorKind<BinaryOperator>
where
    <BinaryOperator as BinaryOperandMetadata<RIGHT_BINARY_OPERAND_POSITION>>::Name:
        std::fmt::Display,
    BinaryOperandOrderValidationError<
        BinaryOperator,
        RIGHT_BINARY_OPERAND_POSITION,
    >: std::fmt::Display,
{
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            BinaryRightOperandValidationErrorKind::Contents(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BinaryRightOperandValidationErrorKind::Order(error) => {
                std::fmt::Display::fmt(error, formatter)
            }
            BinaryRightOperandValidationErrorKind::OperatorLexicalConflict(
                operator,
            ) => {
                write!(
                    formatter,
                    "{} lexically conflicts with the operator",
                    operator.to_operand_name()
                )
            }
        }
    }
}
