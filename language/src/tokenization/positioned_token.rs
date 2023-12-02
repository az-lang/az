use super::token::Token;
use super::types::SubstringPosition;

#[derive(Debug, Eq, PartialEq)]
pub struct PositionedToken<'a> {
    pub position: SubstringPosition,
    pub token: Token<'a>,
}
