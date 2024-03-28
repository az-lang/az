use super::binary_operator::BinaryOperator;
use super::operators::{
    BinaryAnnotationOperator, BinaryArithmeticOperator,
    BinaryAssignmentOperator, BinaryComparisonOperator, CallOperator,
    MemberAccessOperator, UnaryArithmeticOperator,
};
use super::unary_operator::UnaryOperator;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Precedence(u64);

impl Precedence {
    pub(super) fn minimum() -> Self {
        Self(0u64)
    }

    pub(super) fn increment(&self) -> Self {
        Self(self.0 + 1u64)
    }
}

impl From<BinaryAnnotationOperator> for Precedence {
    fn from(_value: BinaryAnnotationOperator) -> Self {
        Precedence(1u64)
    }
}

impl From<BinaryArithmeticOperator> for Precedence {
    fn from(value: BinaryArithmeticOperator) -> Self {
        match value {
            BinaryArithmeticOperator::Addition
            | BinaryArithmeticOperator::Subtraction => Precedence(3u64),
            BinaryArithmeticOperator::Division
            | BinaryArithmeticOperator::Multiplication => Precedence(4u64),
        }
    }
}

impl From<BinaryAssignmentOperator> for Precedence {
    fn from(_value: BinaryAssignmentOperator) -> Self {
        Precedence(0u64)
    }
}

impl From<BinaryComparisonOperator> for Precedence {
    fn from(value: BinaryComparisonOperator) -> Self {
        match value {
            BinaryComparisonOperator::EqualTo
            | BinaryComparisonOperator::GreaterThan
            | BinaryComparisonOperator::GreaterThanOrEqualTo
            | BinaryComparisonOperator::LowerThan
            | BinaryComparisonOperator::LowerThanOrEqualTo
            | BinaryComparisonOperator::NotEqualTo => Precedence(2u64),
        }
    }
}

impl From<CallOperator> for Precedence {
    fn from(_value: CallOperator) -> Self {
        Precedence(6u64)
    }
}

impl From<MemberAccessOperator> for Precedence {
    fn from(_value: MemberAccessOperator) -> Self {
        Precedence(6u64)
    }
}

impl From<UnaryArithmeticOperator> for Precedence {
    fn from(value: UnaryArithmeticOperator) -> Self {
        match value {
            UnaryArithmeticOperator::Negation => Precedence(5u64),
        }
    }
}

impl From<BinaryOperator> for Precedence {
    fn from(value: BinaryOperator) -> Self {
        match value {
            BinaryOperator::Annotation => BinaryAnnotationOperator.into(),
            BinaryOperator::Assignment => BinaryAssignmentOperator.into(),
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
            BinaryOperator::NotEqualTo => {
                BinaryComparisonOperator::NotEqualTo.into()
            }
            BinaryOperator::Addition => {
                BinaryArithmeticOperator::Addition.into()
            }
            BinaryOperator::Division => {
                BinaryArithmeticOperator::Division.into()
            }
            BinaryOperator::Subtraction => {
                BinaryArithmeticOperator::Subtraction.into()
            }
            BinaryOperator::Multiplication => {
                BinaryArithmeticOperator::Multiplication.into()
            }
            BinaryOperator::Call => CallOperator.into(),
            BinaryOperator::MemberAccess => MemberAccessOperator.into(),
        }
    }
}

impl From<UnaryOperator> for Precedence {
    fn from(value: UnaryOperator) -> Self {
        match value {
            UnaryOperator::Negation => {
                UnaryArithmeticOperator::Negation.into()
            }
        }
    }
}
