use crate::tokenization::TokenContent;

use super::associativity::Associativity;
use super::operators::{
    AnnotationOperator, AssignmentOperator, BinaryArithmeticOperator,
    BinaryComparisonOperator, CallOperator, FunctionTypeOperator,
    MemberAccessOperator,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub(super) enum BinaryOperator {
    Addition,
    Annotation,
    Assignment,
    Call,
    Division,
    EqualTo,
    FunctionType,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    MemberAccess,
    Multiplication,
    NotEqualTo,
    Subtraction,
}

impl From<BinaryOperator> for Associativity {
    fn from(value: BinaryOperator) -> Self {
        match value {
            BinaryOperator::Addition => {
                BinaryArithmeticOperator::Addition.into()
            }
            BinaryOperator::Annotation => AnnotationOperator.into(),
            BinaryOperator::Assignment => AssignmentOperator.into(),
            BinaryOperator::Call => CallOperator.into(),
            BinaryOperator::Division => {
                BinaryArithmeticOperator::Division.into()
            }
            BinaryOperator::EqualTo => {
                BinaryComparisonOperator::EqualTo.into()
            }
            BinaryOperator::FunctionType => FunctionTypeOperator.into(),
            BinaryOperator::GreaterThan => {
                BinaryComparisonOperator::GreaterThan.into()
            }
            BinaryOperator::GreaterThanOrEqualTo => {
                BinaryComparisonOperator::GreaterThanOrEqualTo.into()
            }
            BinaryOperator::LessThan => {
                BinaryComparisonOperator::LessThan.into()
            }
            BinaryOperator::LessThanOrEqualTo => {
                BinaryComparisonOperator::LessThanOrEqualTo.into()
            }
            BinaryOperator::MemberAccess => MemberAccessOperator.into(),
            BinaryOperator::Multiplication => {
                BinaryArithmeticOperator::Multiplication.into()
            }
            BinaryOperator::NotEqualTo => {
                BinaryComparisonOperator::NotEqualTo.into()
            }
            BinaryOperator::Subtraction => {
                BinaryArithmeticOperator::Subtraction.into()
            }
        }
    }
}

impl<StringType: AsRef<str>> TryFrom<&TokenContent<StringType>>
    for BinaryOperator
{
    type Error = ();

    fn try_from(
        value: &TokenContent<StringType>,
    ) -> Result<Self, Self::Error> {
        const ADDITION_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            BinaryArithmeticOperator::Addition.into_const_token_content();
        const ANNOTATION_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            AnnotationOperator.into_const_token_content();
        const ASSIGNMENT_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            AssignmentOperator.into_const_token_content();
        const CALL_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            CallOperator.into_const_token_content();
        const DIVISION_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            BinaryArithmeticOperator::Division.into_const_token_content();
        const EQUAL_TO_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            BinaryComparisonOperator::EqualTo.into_const_token_content();
        const FUNCTION_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            FunctionTypeOperator.into_const_token_content();
        const GREATER_THAN_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            BinaryComparisonOperator::GreaterThan.into_const_token_content();
        const GREATER_THAN_OR_EQUAL_TO_OPERATOR_TOKEN_CONTENT: TokenContent<
            &str,
        > = BinaryComparisonOperator::GreaterThanOrEqualTo
            .into_const_token_content();
        const LESS_THAN_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            BinaryComparisonOperator::LessThan.into_const_token_content();
        const LESS_THAN_OR_EQUAL_TO_OPERATOR_TOKEN_CONTENT: TokenContent<
            &str,
        > = BinaryComparisonOperator::LessThanOrEqualTo
            .into_const_token_content();
        const MEMBER_ACCESS_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            MemberAccessOperator.into_const_token_content();
        const MULTIPLICATION_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            BinaryArithmeticOperator::Multiplication
                .into_const_token_content();
        const NOT_EQUAL_TO_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            BinaryComparisonOperator::NotEqualTo.into_const_token_content();
        const SUBTRACTION_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            BinaryArithmeticOperator::Subtraction.into_const_token_content();
        match value.as_ref() {
            ADDITION_OPERATOR_TOKEN_CONTENT => Ok(Self::Addition),
            ANNOTATION_OPERATOR_TOKEN_CONTENT => Ok(Self::Annotation),
            ASSIGNMENT_OPERATOR_TOKEN_CONTENT => Ok(Self::Assignment),
            CALL_OPERATOR_TOKEN_CONTENT => Ok(Self::Call),
            DIVISION_OPERATOR_TOKEN_CONTENT => Ok(Self::Division),
            EQUAL_TO_OPERATOR_TOKEN_CONTENT => Ok(Self::EqualTo),
            FUNCTION_OPERATOR_TOKEN_CONTENT => Ok(Self::FunctionType),
            MULTIPLICATION_OPERATOR_TOKEN_CONTENT => Ok(Self::Multiplication),
            MEMBER_ACCESS_OPERATOR_TOKEN_CONTENT => Ok(Self::MemberAccess),
            SUBTRACTION_OPERATOR_TOKEN_CONTENT => Ok(Self::Subtraction),
            GREATER_THAN_OPERATOR_TOKEN_CONTENT => Ok(Self::GreaterThan),
            GREATER_THAN_OR_EQUAL_TO_OPERATOR_TOKEN_CONTENT => {
                Ok(Self::GreaterThanOrEqualTo)
            }
            LESS_THAN_OPERATOR_TOKEN_CONTENT => Ok(Self::LessThan),
            LESS_THAN_OR_EQUAL_TO_OPERATOR_TOKEN_CONTENT => {
                Ok(Self::LessThanOrEqualTo)
            }
            NOT_EQUAL_TO_OPERATOR_TOKEN_CONTENT => Ok(Self::NotEqualTo),
            _ => Err(()),
        }
    }
}
