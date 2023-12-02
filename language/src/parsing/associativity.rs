use super::binary_operation::BinaryOperator;

pub(super) enum Associativity {
    LeftToRight,
    RightToLeft,
}

impl From<BinaryOperator> for Associativity {
    fn from(value: BinaryOperator) -> Self {
        match value {
            BinaryOperator::Annotation | BinaryOperator::Assignment => {
                Self::RightToLeft
            }
            BinaryOperator::Addition
            | BinaryOperator::Division
            | BinaryOperator::EqualTo
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanOrEqualTo
            | BinaryOperator::LowerThan
            | BinaryOperator::LowerThanOrEqualTo
            | BinaryOperator::MemberAccess
            | BinaryOperator::Multiplication
            | BinaryOperator::NotEqualTo
            | BinaryOperator::Subtraction => Self::LeftToRight,
        }
    }
}
