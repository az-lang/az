use std::fmt::{Display, Formatter, Write};

use super::constants::{
    ARROW, ASSIGNMENT, ASTERISK, CLOSE_BRACE, CLOSE_PARENTHESES, COLON, COMMA,
    DOT, EQUAL_TO, GREATER_THAN, GREATER_THAN_OR_EQUAL_TO, LOWER_THAN,
    LOWER_THAN_OR_EQUAL_TO, MINUS, NEWLINE, NOT_EQUAL_TO, OPEN_BRACE,
    OPEN_PARENTHESES, PLUS, SEMICOLON, SLASH, TYPE_SUFFIX_SEPARATOR,
};
use super::numeric_literal_type::NumericLiteralType;

#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Arrow,
    Assignment,
    Asterisk,
    CloseBrace,
    CloseParenthesis,
    Colon,
    Comma,
    CommentBlock(Vec<&'a str>),
    CommentLine(&'a str),
    Dot,
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    NumericLiteral {
        value: &'a str,
        type_: NumericLiteralType,
    },
    Identifier(&'a str),
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
    Whitespace(&'a str),
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Arrow => formatter.write_str(ARROW),
            Token::Assignment => formatter.write_char(ASSIGNMENT),
            Token::Asterisk => formatter.write_char(ASTERISK),
            Token::CloseBrace => formatter.write_char(CLOSE_BRACE),
            Token::CloseParenthesis => formatter.write_char(CLOSE_PARENTHESES),
            Token::Colon => formatter.write_char(COLON),
            Token::Comma => formatter.write_char(COMMA),
            Token::CommentBlock(lines) => {
                for line in lines {
                    formatter.write_str(line)?;
                }
                Ok(())
            }
            Token::CommentLine(line) => formatter.write_str(line),
            Token::Dot => formatter.write_char(DOT),
            Token::EqualTo => formatter.write_str(EQUAL_TO),
            Token::GreaterThan => formatter.write_char(GREATER_THAN),
            Token::GreaterThanOrEqualTo => {
                formatter.write_str(GREATER_THAN_OR_EQUAL_TO)
            }
            Token::Identifier(string) => formatter.write_str(string),
            Token::LowerThan => formatter.write_char(LOWER_THAN),
            Token::LowerThanOrEqualTo => {
                formatter.write_str(LOWER_THAN_OR_EQUAL_TO)
            }
            Token::Minus => formatter.write_char(MINUS),
            Token::Newline => formatter.write_char(NEWLINE),
            Token::NotEqualTo => formatter.write_str(NOT_EQUAL_TO),
            Token::NumericLiteral { value, type_ } => {
                formatter.write_str(value)?;
                formatter.write_char(TYPE_SUFFIX_SEPARATOR)?;
                formatter.write_str(type_.into())
            }
            Token::OpenBrace => formatter.write_char(OPEN_BRACE),
            Token::OpenParenthesis => formatter.write_char(OPEN_PARENTHESES),
            Token::Plus => formatter.write_char(PLUS),
            Token::Semicolon => formatter.write_char(SEMICOLON),
            Token::Slash => formatter.write_char(SLASH),
            Token::Whitespace(string) => formatter.write_str(string),
        }
    }
}
