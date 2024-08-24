use crate::parsing::Associativity;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FunctionTypeOperator;

impl FunctionTypeOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        TokenContent::Arrow
    }
}

impl From<FunctionTypeOperator> for Associativity {
    fn from(_value: FunctionTypeOperator) -> Self {
        Self::LeftToRight
    }
}

impl<StringType> From<FunctionTypeOperator> for TokenContent<StringType> {
    fn from(_value: FunctionTypeOperator) -> Self {
        TokenContent::Arrow
    }
}
