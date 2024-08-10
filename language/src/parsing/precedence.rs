use super::binary_operator::BinaryOperator;
use super::operators::{
    AnnotationOperator, AssignmentOperator, BinaryArithmeticOperator,
    BinaryComparisonOperator, CallOperator, FunctionTypeOperator,
    MemberAccessOperator, ReturnOperator, UnaryArithmeticOperator,
};
use super::unary_operator::UnaryOperator;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Precedence(u64);

impl Precedence {
    pub(super) const fn minimum() -> Self {
        Self(0u64)
    }

    pub(super) const fn increment(&self) -> Self {
        Self(self.0 + 1u64)
    }
}

const RETURN_OPERATOR_PRECEDENCE: Precedence = Precedence::minimum();
const BINARY_ASSIGNMENT_OPERATOR_PRECEDENCE: Precedence =
    RETURN_OPERATOR_PRECEDENCE.increment();
const BINARY_ANNOTATION_OPERATOR_PRECEDENCE: Precedence =
    BINARY_ASSIGNMENT_OPERATOR_PRECEDENCE.increment();
const FUNCTION_TYPE_OPERATOR_PRECEDENCE: Precedence =
    BINARY_ANNOTATION_OPERATOR_PRECEDENCE.increment();
const BINARY_COMPARISON_OPERATOR_PRECEDENCE: Precedence =
    FUNCTION_TYPE_OPERATOR_PRECEDENCE.increment();
const BINARY_ARITHMETIC_ADDITION_SUBTRACTION_PRECEDENCE: Precedence =
    BINARY_COMPARISON_OPERATOR_PRECEDENCE.increment();
const BINARY_ARITHMETIC_DIVISION_MULTIPLICATION_PRECEDENCE: Precedence =
    BINARY_ARITHMETIC_ADDITION_SUBTRACTION_PRECEDENCE.increment();
const MAX_BINARY_ARITHMETIC_OPERATOR_PRECEDENCE: Precedence =
    BINARY_ARITHMETIC_DIVISION_MULTIPLICATION_PRECEDENCE;
const UNARY_ARITHMETIC_OPERATOR_PRECEDENCE: Precedence =
    MAX_BINARY_ARITHMETIC_OPERATOR_PRECEDENCE.increment();
const CALL_MEMBER_ACCESS_OPERATOR_PRECEDENCE: Precedence =
    UNARY_ARITHMETIC_OPERATOR_PRECEDENCE.increment();

impl From<AnnotationOperator> for Precedence {
    fn from(_value: AnnotationOperator) -> Self {
        BINARY_ANNOTATION_OPERATOR_PRECEDENCE
    }
}

impl From<BinaryArithmeticOperator> for Precedence {
    fn from(value: BinaryArithmeticOperator) -> Self {
        match value {
            BinaryArithmeticOperator::Addition
            | BinaryArithmeticOperator::Subtraction => {
                BINARY_ARITHMETIC_ADDITION_SUBTRACTION_PRECEDENCE
            }
            BinaryArithmeticOperator::Division
            | BinaryArithmeticOperator::Multiplication => {
                BINARY_ARITHMETIC_DIVISION_MULTIPLICATION_PRECEDENCE
            }
        }
    }
}

impl From<AssignmentOperator> for Precedence {
    fn from(_value: AssignmentOperator) -> Self {
        BINARY_ASSIGNMENT_OPERATOR_PRECEDENCE
    }
}

impl From<BinaryComparisonOperator> for Precedence {
    fn from(value: BinaryComparisonOperator) -> Self {
        match value {
            BinaryComparisonOperator::EqualTo
            | BinaryComparisonOperator::GreaterThan
            | BinaryComparisonOperator::GreaterThanOrEqualTo
            | BinaryComparisonOperator::LessThan
            | BinaryComparisonOperator::LessThanOrEqualTo
            | BinaryComparisonOperator::NotEqualTo => {
                BINARY_COMPARISON_OPERATOR_PRECEDENCE
            }
        }
    }
}

impl From<CallOperator> for Precedence {
    fn from(_value: CallOperator) -> Self {
        CALL_MEMBER_ACCESS_OPERATOR_PRECEDENCE
    }
}

impl From<FunctionTypeOperator> for Precedence {
    fn from(_value: FunctionTypeOperator) -> Self {
        FUNCTION_TYPE_OPERATOR_PRECEDENCE
    }
}

impl From<MemberAccessOperator> for Precedence {
    fn from(_value: MemberAccessOperator) -> Self {
        CALL_MEMBER_ACCESS_OPERATOR_PRECEDENCE
    }
}

impl From<ReturnOperator> for Precedence {
    fn from(_value: ReturnOperator) -> Self {
        RETURN_OPERATOR_PRECEDENCE
    }
}

impl From<UnaryArithmeticOperator> for Precedence {
    fn from(value: UnaryArithmeticOperator) -> Self {
        match value {
            UnaryArithmeticOperator::Negation => {
                UNARY_ARITHMETIC_OPERATOR_PRECEDENCE
            }
        }
    }
}

impl From<BinaryOperator> for Precedence {
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

impl From<UnaryOperator> for Precedence {
    fn from(value: UnaryOperator) -> Self {
        match value {
            UnaryOperator::Negation => {
                UnaryArithmeticOperator::Negation.into()
            }
            UnaryOperator::Return => RETURN_OPERATOR_PRECEDENCE,
        }
    }
}
