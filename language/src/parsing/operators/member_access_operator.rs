use crate::parsing::Associativity;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemberAccessOperator;

impl MemberAccessOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        TokenContent::Dot
    }
}

impl From<MemberAccessOperator> for Associativity {
    fn from(_value: MemberAccessOperator) -> Self {
        Self::LeftToRight
    }
}

impl<StringType> From<MemberAccessOperator> for TokenContent<StringType> {
    fn from(_value: MemberAccessOperator) -> Self {
        TokenContent::Dot
    }
}
