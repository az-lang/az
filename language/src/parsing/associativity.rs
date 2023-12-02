use super::binary_operator::BinaryOperator;
use super::operators::{
    BinaryAnnotationOperator, BinaryArithmeticOperator,
    BinaryAssignmentOperator, BinaryComparisonOperator, CallOperator,
    MemberAccessOperator,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

impl From<BinaryAnnotationOperator> for Associativity {
    fn from(_value: BinaryAnnotationOperator) -> Self {
        Self::RightToLeft
    }
}

impl From<BinaryAssignmentOperator> for Associativity {
    fn from(_value: BinaryAssignmentOperator) -> Self {
        Self::RightToLeft
    }
}

impl From<BinaryArithmeticOperator> for Associativity {
    fn from(value: BinaryArithmeticOperator) -> Self {
        match value {
            BinaryArithmeticOperator::Addition
            | BinaryArithmeticOperator::Division
            | BinaryArithmeticOperator::Multiplication
            | BinaryArithmeticOperator::Subtraction => Self::LeftToRight,
        }
    }
}

impl From<BinaryComparisonOperator> for Associativity {
    fn from(value: BinaryComparisonOperator) -> Self {
        match value {
            BinaryComparisonOperator::EqualTo
            | BinaryComparisonOperator::GreaterThan
            | BinaryComparisonOperator::GreaterThanOrEqualTo
            | BinaryComparisonOperator::LowerThan
            | BinaryComparisonOperator::LowerThanOrEqualTo
            | BinaryComparisonOperator::NotEqualTo => Self::LeftToRight,
        }
    }
}

impl From<BinaryOperator> for Associativity {
    fn from(value: BinaryOperator) -> Self {
        match value {
            BinaryOperator::Annotation => BinaryAnnotationOperator.into(),
            BinaryOperator::Assignment => BinaryAssignmentOperator.into(),
            BinaryOperator::Addition => {
                BinaryArithmeticOperator::Addition.into()
            }
            BinaryOperator::Division => {
                BinaryArithmeticOperator::Division.into()
            }
            BinaryOperator::Multiplication => {
                BinaryArithmeticOperator::Multiplication.into()
            }
            BinaryOperator::Subtraction => {
                BinaryArithmeticOperator::Subtraction.into()
            }
            BinaryOperator::Call => CallOperator.into(),
            BinaryOperator::EqualTo => {
                BinaryComparisonOperator::EqualTo.into()
            }
            BinaryOperator::GreaterThan => {
                BinaryComparisonOperator::GreaterThan.into()
            }
            BinaryOperator::GreaterThanOrEqualTo => {
                BinaryComparisonOperator::GreaterThanOrEqualTo.into()
            }
            BinaryOperator::LowerThan => {
                BinaryComparisonOperator::LowerThan.into()
            }
            BinaryOperator::LowerThanOrEqualTo => {
                BinaryComparisonOperator::LowerThanOrEqualTo.into()
            }
            BinaryOperator::MemberAccess => MemberAccessOperator.into(),
            BinaryOperator::NotEqualTo => {
                BinaryComparisonOperator::NotEqualTo.into()
            }
        }
    }
}

impl From<CallOperator> for Associativity {
    fn from(_value: CallOperator) -> Self {
        Self::LeftToRight
    }
}

impl From<MemberAccessOperator> for Associativity {
    fn from(_value: MemberAccessOperator) -> Self {
        Self::LeftToRight
    }
}
