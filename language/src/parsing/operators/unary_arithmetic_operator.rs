use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnaryArithmeticOperator {
    Negation,
}

impl UnaryArithmeticOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        match self {
            UnaryArithmeticOperator::Negation => TokenContent::Minus,
        }
    }
}

impl<StringType> From<UnaryArithmeticOperator> for TokenContent<StringType> {
    fn from(value: UnaryArithmeticOperator) -> Self {
        match value {
            UnaryArithmeticOperator::Negation => TokenContent::Minus,
        }
    }
}
