use crate::parsing::{ReturnOperator, UnaryArithmeticOperator};
use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub(super) enum UnaryOperator {
    Negation,
    Return,
}

impl<StringType: AsRef<str>> TryFrom<&TokenContent<StringType>>
    for UnaryOperator
{
    type Error = ();

    fn try_from(
        value: &TokenContent<StringType>,
    ) -> Result<Self, Self::Error> {
        const NEGATION_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            UnaryArithmeticOperator::Negation.into_const_token_content();
        const RETURN_OPERATOR_TOKEN_CONTENT: TokenContent<&str> =
            ReturnOperator.into_const_token_content();
        match value.as_ref() {
            NEGATION_OPERATOR_TOKEN_CONTENT => Ok(Self::Negation),
            RETURN_OPERATOR_TOKEN_CONTENT => Ok(Self::Return),
            _ => Err(()),
        }
    }
}
