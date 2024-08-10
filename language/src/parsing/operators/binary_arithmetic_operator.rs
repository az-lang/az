use crate::parsing::Associativity;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryArithmeticOperator {
    Addition,
    Division,
    Multiplication,
    Subtraction,
}

impl BinaryArithmeticOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        match self {
            BinaryArithmeticOperator::Addition => TokenContent::Plus,
            BinaryArithmeticOperator::Division => TokenContent::Slash,
            BinaryArithmeticOperator::Multiplication => TokenContent::Asterisk,
            BinaryArithmeticOperator::Subtraction => TokenContent::Minus,
        }
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
