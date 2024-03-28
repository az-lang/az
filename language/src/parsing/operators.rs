use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinaryAnnotationOperator;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryArithmeticOperator {
    Addition,
    Division,
    Multiplication,
    Subtraction,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinaryAssignmentOperator;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryComparisonOperator {
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LowerThan,
    LowerThanOrEqualTo,
    NotEqualTo,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CallOperator;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemberAccessOperator;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnaryArithmeticOperator {
    Negation,
}

impl<StringType> From<BinaryAnnotationOperator> for TokenContent<StringType> {
    fn from(_value: BinaryAnnotationOperator) -> Self {
        TokenContent::Colon
    }
}

impl<StringType> From<BinaryAssignmentOperator> for TokenContent<StringType> {
    fn from(_value: BinaryAssignmentOperator) -> Self {
        TokenContent::Assignment
    }
}

impl<StringType> From<BinaryArithmeticOperator> for TokenContent<StringType> {
    fn from(value: BinaryArithmeticOperator) -> Self {
        match value {
            BinaryArithmeticOperator::Addition => TokenContent::Plus,
            BinaryArithmeticOperator::Division => TokenContent::Slash,
            BinaryArithmeticOperator::Multiplication => TokenContent::Asterisk,
            BinaryArithmeticOperator::Subtraction => TokenContent::Minus,
        }
    }
}

impl<StringType> From<BinaryComparisonOperator> for TokenContent<StringType> {
    fn from(value: BinaryComparisonOperator) -> Self {
        match value {
            BinaryComparisonOperator::EqualTo => TokenContent::EqualTo,
            BinaryComparisonOperator::GreaterThan => TokenContent::GreaterThan,
            BinaryComparisonOperator::GreaterThanOrEqualTo => {
                TokenContent::GreaterThanOrEqualTo
            }
            BinaryComparisonOperator::LowerThan => TokenContent::LowerThan,
            BinaryComparisonOperator::LowerThanOrEqualTo => {
                TokenContent::LowerThanOrEqualTo
            }
            BinaryComparisonOperator::NotEqualTo => TokenContent::NotEqualTo,
        }
    }
}

impl<StringType> From<MemberAccessOperator> for TokenContent<StringType> {
    fn from(_value: MemberAccessOperator) -> Self {
        TokenContent::Dot
    }
}

impl<StringType> From<UnaryArithmeticOperator> for TokenContent<StringType> {
    fn from(value: UnaryArithmeticOperator) -> Self {
        match value {
            UnaryArithmeticOperator::Negation => TokenContent::Minus,
        }
    }
}
