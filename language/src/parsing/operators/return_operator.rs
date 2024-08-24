use crate::parsing::RETURN_OPERATOR_STRING;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReturnOperator;

impl<StringType: From<&'static str>> From<ReturnOperator>
    for TokenContent<StringType>
{
    fn from(_: ReturnOperator) -> Self {
        TokenContent::Identifier(RETURN_OPERATOR_STRING.into())
    }
}

impl ReturnOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        TokenContent::Identifier(RETURN_OPERATOR_STRING)
    }
}
