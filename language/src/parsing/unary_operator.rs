use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub(super) enum UnaryOperator {
    Negation,
}

impl<StringType> TryFrom<&TokenContent<StringType>> for UnaryOperator {
    type Error = ();

    fn try_from(
        value: &TokenContent<StringType>,
    ) -> Result<Self, Self::Error> {
        match value {
            TokenContent::Minus => Ok(Self::Negation),
            _ => Err(()),
        }
    }
}
