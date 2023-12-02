use crate::parsing::binary_operation::BinaryOperator;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct Precedence(u64);

impl Precedence {
    pub(super) fn increment(self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u64> for Precedence {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<BinaryOperator> for Precedence {
    fn from(value: BinaryOperator) -> Self {
        match value {
            BinaryOperator::Annotation | BinaryOperator::Assignment => Self(0),
            BinaryOperator::EqualTo
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanOrEqualTo
            | BinaryOperator::LowerThan
            | BinaryOperator::LowerThanOrEqualTo
            | BinaryOperator::NotEqualTo => Self(1),
            BinaryOperator::Addition | BinaryOperator::Subtraction => Self(2),
            BinaryOperator::Division | BinaryOperator::Multiplication => {
                Self(3)
            }
            BinaryOperator::MemberAccess => Self(4),
        }
    }
}
