use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::traits::TryToJsString;
use az::tokenization::{NumericLiteralType, TokenContent};

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[wasm_bindgen(js_name = "TokenKind")]
pub enum JsTokenKind {
    ARROW,
    ASSIGNMENT,
    ASTERISK,
    CLOSE_BRACE,
    CLOSE_PARENTHESIS,
    COLON,
    COMMA,
    COMMENT_BLOCK,
    COMMENT_LINE,
    DOT,
    EQUAL_TO,
    F32,
    F64,
    GREATER_THAN,
    GREATER_THAN_OR_EQUAL_TO,
    I8,
    I16,
    I32,
    I64,
    IDENTIFIER,
    ISIZE,
    LESS_THAN,
    LESS_THAN_OR_EQUAL_TO,
    MINUS,
    NEWLINE,
    NOT_EQUAL_TO,
    OPEN_BRACE,
    OPEN_PARENTHESIS,
    PLUS,
    SEMICOLON,
    SLASH,
    U8,
    U16,
    U32,
    U64,
    USIZE,
    WHITESPACE,
}

impl<StringType> From<&TokenContent<StringType>> for JsTokenKind {
    fn from(value: &TokenContent<StringType>) -> Self {
        match value {
            TokenContent::Arrow => Self::ARROW,
            TokenContent::Assignment => Self::ASSIGNMENT,
            TokenContent::Asterisk => Self::ASTERISK,
            TokenContent::CloseBrace => Self::CLOSE_BRACE,
            TokenContent::CloseParenthesis => Self::CLOSE_PARENTHESIS,
            TokenContent::Colon => Self::COLON,
            TokenContent::Comma => Self::COMMA,
            TokenContent::CommentBlock(_) => Self::COMMENT_BLOCK,
            TokenContent::CommentLine(_) => Self::COMMENT_LINE,
            TokenContent::Dot => Self::DOT,
            TokenContent::EqualTo => Self::EQUAL_TO,
            TokenContent::GreaterThan => Self::GREATER_THAN,
            TokenContent::GreaterThanOrEqualTo => {
                Self::GREATER_THAN_OR_EQUAL_TO
            }
            TokenContent::NumericLiteral { type_, .. } => match type_ {
                NumericLiteralType::F32 => Self::F32,
                NumericLiteralType::F64 => Self::F64,
                NumericLiteralType::I8 => Self::I8,
                NumericLiteralType::I16 => Self::I16,
                NumericLiteralType::I32 => Self::I32,
                NumericLiteralType::I64 => Self::I64,
                NumericLiteralType::ISize => Self::ISIZE,
                NumericLiteralType::U8 => Self::U8,
                NumericLiteralType::U16 => Self::U16,
                NumericLiteralType::U32 => Self::U32,
                NumericLiteralType::U64 => Self::U64,
                NumericLiteralType::USize => Self::USIZE,
            },
            TokenContent::Identifier(_) => Self::IDENTIFIER,
            TokenContent::LessThan => Self::LESS_THAN,
            TokenContent::LessThanOrEqualTo => Self::LESS_THAN_OR_EQUAL_TO,
            TokenContent::Minus => Self::MINUS,
            TokenContent::Newline => Self::NEWLINE,
            TokenContent::NotEqualTo => Self::NOT_EQUAL_TO,
            TokenContent::OpenBrace => Self::OPEN_BRACE,
            TokenContent::OpenParenthesis => Self::OPEN_PARENTHESIS,
            TokenContent::Plus => Self::PLUS,
            TokenContent::Semicolon => Self::SEMICOLON,
            TokenContent::Slash => Self::SLASH,
            TokenContent::Whitespace(_) => Self::WHITESPACE,
        }
    }
}

impl TryToJsString for JsTokenKind {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "TokenKind.{}",
            match self {
                JsTokenKind::ARROW => "ARROW",
                JsTokenKind::ASSIGNMENT => "ASSIGNMENT",
                JsTokenKind::ASTERISK => "ASTERISK",
                JsTokenKind::CLOSE_BRACE => "CLOSE_BRACE",
                JsTokenKind::CLOSE_PARENTHESIS => "CLOSE_PARENTHESIS",
                JsTokenKind::COLON => "COLON",
                JsTokenKind::COMMA => "COMMA",
                JsTokenKind::COMMENT_BLOCK => "COMMENT_BLOCK",
                JsTokenKind::COMMENT_LINE => "COMMENT_LINE",
                JsTokenKind::DOT => "DOT",
                JsTokenKind::EQUAL_TO => "EQUAL_TO",
                JsTokenKind::F32 => "F32",
                JsTokenKind::F64 => "F64",
                JsTokenKind::GREATER_THAN => "GREATER_THAN",
                JsTokenKind::GREATER_THAN_OR_EQUAL_TO => {
                    "GREATER_THAN_OR_EQUAL_TO"
                }
                JsTokenKind::I8 => "I8",
                JsTokenKind::I16 => "I16",
                JsTokenKind::I32 => "I32",
                JsTokenKind::I64 => "I64",
                JsTokenKind::IDENTIFIER => "IDENTIFIER",
                JsTokenKind::ISIZE => "ISIZE",
                JsTokenKind::LESS_THAN => "LESS_THAN",
                JsTokenKind::LESS_THAN_OR_EQUAL_TO => "LESS_THAN_OR_EQUAL_TO",
                JsTokenKind::MINUS => "MINUS",
                JsTokenKind::NEWLINE => "NEWLINE",
                JsTokenKind::NOT_EQUAL_TO => "NOT_EQUAL_TO",
                JsTokenKind::OPEN_BRACE => "OPEN_BRACE",
                JsTokenKind::OPEN_PARENTHESIS => "OPEN_PARENTHESIS",
                JsTokenKind::PLUS => "PLUS",
                JsTokenKind::SEMICOLON => "SEMICOLON",
                JsTokenKind::SLASH => "SLASH",
                JsTokenKind::U8 => "U8",
                JsTokenKind::U16 => "U16",
                JsTokenKind::U32 => "U32",
                JsTokenKind::U64 => "U64",
                JsTokenKind::USIZE => "USIZE",
                JsTokenKind::WHITESPACE => "WHITESPACE",
            }
        ))
    }
}
