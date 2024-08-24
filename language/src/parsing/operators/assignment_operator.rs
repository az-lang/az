use crate::parsing::Associativity;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssignmentOperator;

impl AssignmentOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        TokenContent::Assignment
    }
}

impl From<AssignmentOperator> for Associativity {
    fn from(_value: AssignmentOperator) -> Self {
        Self::RightToLeft
    }
}

impl<StringType> From<AssignmentOperator> for TokenContent<StringType> {
    fn from(_value: AssignmentOperator) -> Self {
        TokenContent::Assignment
    }
}
