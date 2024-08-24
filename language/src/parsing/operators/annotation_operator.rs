use crate::parsing::Associativity;
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnnotationOperator;

impl AnnotationOperator {
    pub(crate) const fn into_const_token_content(
        self,
    ) -> TokenContent<&'static str> {
        TokenContent::Colon
    }
}

impl From<AnnotationOperator> for Associativity {
    fn from(_value: AnnotationOperator) -> Self {
        Self::RightToLeft
    }
}

impl<StringType> From<AnnotationOperator> for TokenContent<StringType> {
    fn from(_value: AnnotationOperator) -> Self {
        TokenContent::Colon
    }
}
