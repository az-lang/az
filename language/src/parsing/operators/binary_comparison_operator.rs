use crate::parsing::Associativity;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryComparisonOperator {
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    NotEqualTo,
}

impl BinaryComparisonOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        match self {
            Self::EqualTo => TokenContent::EqualTo,
            Self::GreaterThan => TokenContent::GreaterThan,
            Self::GreaterThanOrEqualTo => TokenContent::GreaterThanOrEqualTo,
            Self::LessThan => TokenContent::LessThan,
            Self::LessThanOrEqualTo => TokenContent::LessThanOrEqualTo,
            Self::NotEqualTo => TokenContent::NotEqualTo,
        }
    }
}

impl From<BinaryComparisonOperator> for Associativity {
    fn from(value: BinaryComparisonOperator) -> Self {
        match value {
            BinaryComparisonOperator::EqualTo
            | BinaryComparisonOperator::GreaterThan
            | BinaryComparisonOperator::GreaterThanOrEqualTo
            | BinaryComparisonOperator::LessThan
            | BinaryComparisonOperator::LessThanOrEqualTo
            | BinaryComparisonOperator::NotEqualTo => Self::LeftToRight,
        }
    }
}

impl<StringType> From<BinaryComparisonOperator> for TokenContent<StringType> {
    fn from(value: BinaryComparisonOperator) -> Self {
        match value {
            BinaryComparisonOperator::EqualTo => Self::EqualTo,
            BinaryComparisonOperator::GreaterThan => Self::GreaterThan,
            BinaryComparisonOperator::GreaterThanOrEqualTo => {
                Self::GreaterThanOrEqualTo
            }
            BinaryComparisonOperator::LessThan => Self::LessThan,
            BinaryComparisonOperator::LessThanOrEqualTo => {
                Self::LessThanOrEqualTo
            }
            BinaryComparisonOperator::NotEqualTo => Self::NotEqualTo,
        }
    }
}
