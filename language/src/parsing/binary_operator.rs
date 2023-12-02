use crate::tokenization::TokenContent;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub(super) enum BinaryOperator {
    Addition,
    Annotation,
    Assignment,
    Call,
    Division,
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LowerThan,
    LowerThanOrEqualTo,
    MemberAccess,
    Multiplication,
    NotEqualTo,
    Subtraction,
}

impl<StringType> TryFrom<&TokenContent<StringType>> for BinaryOperator {
    type Error = ();

    fn try_from(
        value: &TokenContent<StringType>,
    ) -> Result<Self, Self::Error> {
        match value {
            TokenContent::Assignment => Ok(Self::Assignment),
            TokenContent::Asterisk => Ok(Self::Multiplication),
            TokenContent::Colon => Ok(Self::Annotation),
            TokenContent::Dot => Ok(Self::MemberAccess),
            TokenContent::EqualTo => Ok(Self::EqualTo),
            TokenContent::GreaterThan => Ok(Self::GreaterThan),
            TokenContent::GreaterThanOrEqualTo => {
                Ok(Self::GreaterThanOrEqualTo)
            }
            TokenContent::LowerThan => Ok(Self::LowerThan),
            TokenContent::LowerThanOrEqualTo => Ok(Self::LowerThanOrEqualTo),
            TokenContent::Minus => Ok(Self::Subtraction),
            TokenContent::NotEqualTo => Ok(Self::NotEqualTo),
            TokenContent::OpenParenthesis => Ok(Self::Call),
            TokenContent::Plus => Ok(Self::Addition),
            TokenContent::Slash => Ok(Self::Division),
            _ => Err(()),
        }
    }
}
