use std::fmt::{Display, Formatter, Write};

use super::constants::{
    ARROW, ASSIGNMENT, ASTERISK, CLOSE_BRACE, CLOSE_PARENTHESES, COLON, COMMA,
    DOT, EQUAL_TO, GREATER_THAN, GREATER_THAN_OR_EQUAL_TO, LOWER_THAN,
    LOWER_THAN_OR_EQUAL_TO, MINUS, NEWLINE, NOT_EQUAL_TO, OPEN_BRACE,
    OPEN_PARENTHESES, PLUS, SEMICOLON, SLASH, TYPE_SUFFIX_SEPARATOR,
};
use super::numeric_literal_type::NumericLiteralType;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenContent<StringType> {
    Arrow,
    Assignment,
    Asterisk,
    CloseBrace,
    CloseParenthesis,
    Colon,
    Comma,
    CommentBlock(Vec<StringType>),
    CommentLine(StringType),
    Dot,
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    NumericLiteral {
        value: StringType,
        type_: NumericLiteralType,
    },
    Identifier(StringType),
    LowerThan,
    LowerThanOrEqualTo,
    Minus,
    Newline,
    NotEqualTo,
    OpenBrace,
    OpenParenthesis,
    Plus,
    Semicolon,
    Slash,
    Whitespace(StringType),
}

impl<StringType: AsRef<str>> Display for TokenContent<StringType> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenContent::Arrow => formatter.write_str(ARROW),
            TokenContent::Assignment => formatter.write_char(ASSIGNMENT),
            TokenContent::Asterisk => formatter.write_char(ASTERISK),
            TokenContent::CloseBrace => formatter.write_char(CLOSE_BRACE),
            TokenContent::CloseParenthesis => {
                formatter.write_char(CLOSE_PARENTHESES)
            }
            TokenContent::Colon => formatter.write_char(COLON),
            TokenContent::Comma => formatter.write_char(COMMA),
            TokenContent::CommentBlock(lines) => {
                for line in lines {
                    formatter.write_str(line.as_ref())?;
                }
                Ok(())
            }
            TokenContent::CommentLine(line) => {
                formatter.write_str(line.as_ref())
            }
            TokenContent::Dot => formatter.write_char(DOT),
            TokenContent::EqualTo => formatter.write_str(EQUAL_TO),
            TokenContent::GreaterThan => formatter.write_char(GREATER_THAN),
            TokenContent::GreaterThanOrEqualTo => {
                formatter.write_str(GREATER_THAN_OR_EQUAL_TO)
            }
            TokenContent::Identifier(string) => {
                formatter.write_str(string.as_ref())
            }
            TokenContent::LowerThan => formatter.write_char(LOWER_THAN),
            TokenContent::LowerThanOrEqualTo => {
                formatter.write_str(LOWER_THAN_OR_EQUAL_TO)
            }
            TokenContent::Minus => formatter.write_char(MINUS),
            TokenContent::Newline => formatter.write_char(NEWLINE),
            TokenContent::NotEqualTo => formatter.write_str(NOT_EQUAL_TO),
            TokenContent::NumericLiteral { value, type_ } => {
                formatter.write_str(value.as_ref())?;
                formatter.write_char(TYPE_SUFFIX_SEPARATOR)?;
                formatter.write_str(type_.into())
            }
            TokenContent::OpenBrace => formatter.write_char(OPEN_BRACE),
            TokenContent::OpenParenthesis => {
                formatter.write_char(OPEN_PARENTHESES)
            }
            TokenContent::Plus => formatter.write_char(PLUS),
            TokenContent::Semicolon => formatter.write_char(SEMICOLON),
            TokenContent::Slash => formatter.write_char(SLASH),
            TokenContent::Whitespace(string) => {
                formatter.write_str(string.as_ref())
            }
        }
    }
}
