use crate::tokenization::{SubstringPosition, Token};

use super::expressions::Expression;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct MismatchedOpenBrace {
    pub position: SubstringPosition,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct MismatchedOpenParenthesis {
    pub position: SubstringPosition,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct MissingSemicolon<TokenStringType> {
    pub token: Token<TokenStringType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedExpression<StringType> {
    pub expression: Expression<StringType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedToken<TokenStringType> {
    pub token: Token<TokenStringType>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutOfTokens;

#[derive(Clone, Debug, PartialEq)]
pub enum ParsingError<StringType, TokenStringType> {
    MismatchedOpenBrace(MismatchedOpenBrace),
    MismatchedOpenParenthesis(MismatchedOpenParenthesis),
    MissingSemicolon(MissingSemicolon<TokenStringType>),
    OutOfTokens(OutOfTokens),
    UnexpectedExpression(UnexpectedExpression<StringType>),
    UnexpectedToken(UnexpectedToken<TokenStringType>),
}
