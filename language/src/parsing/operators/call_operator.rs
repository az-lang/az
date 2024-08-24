use crate::parsing::Associativity;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CallOperator;

impl CallOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        TokenContent::OpenParenthesis
    }
}

impl From<CallOperator> for Associativity {
    fn from(_value: CallOperator) -> Self {
        Self::LeftToRight
    }
}
