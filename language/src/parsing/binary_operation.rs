use crate::tokenization::Token;

#[derive(Clone, Copy, Debug)]
pub(super) enum BinaryOperator {
    Addition,
    Annotation,
    Assignment,
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

impl<'a> TryFrom<&Token<'a>> for BinaryOperator {
    type Error = ();

    fn try_from(value: &Token<'a>) -> Result<Self, Self::Error> {
        match value {
            Token::Assignment => Ok(Self::Assignment),
            Token::Asterisk => Ok(Self::Multiplication),
            Token::Colon => Ok(Self::Annotation),
            Token::Dot => Ok(Self::MemberAccess),
            Token::EqualTo => Ok(Self::EqualTo),
            Token::GreaterThan => Ok(Self::GreaterThan),
            Token::GreaterThanOrEqualTo => Ok(Self::GreaterThanOrEqualTo),
            Token::LowerThan => Ok(Self::LowerThan),
            Token::LowerThanOrEqualTo => Ok(Self::LowerThanOrEqualTo),
            Token::Minus => Ok(Self::Subtraction),
            Token::NotEqualTo => Ok(Self::NotEqualTo),
            Token::Plus => Ok(Self::Addition),
            Token::Slash => Ok(Self::Division),
            _ => Err(()),
        }
    }
}
