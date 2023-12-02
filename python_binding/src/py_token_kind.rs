use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[pyclass(module = "az.tokenization", name = "TokenKind", frozen)]
pub(super) enum PyTokenKind {
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
    LOWER_THAN,
    LOWER_THAN_OR_EQUAL_TO,
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

#[pymethods]
impl PyTokenKind {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl Repr for PyTokenKind {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self {
                Self::ARROW => "ARROW",
                Self::ASSIGNMENT => "ASSIGNMENT",
                Self::ASTERISK => "ASTERISK",
                Self::CLOSE_BRACE => "CLOSE_BRACE",
                Self::CLOSE_PARENTHESIS => "CLOSE_PARENTHESIS",
                Self::COLON => "COLON",
                Self::COMMA => "COMMA",
                Self::COMMENT_BLOCK => "COMMENT_BLOCK",
                Self::COMMENT_LINE => "COMMENT_LINE",
                Self::DOT => "DOT",
                Self::EQUAL_TO => "EQUAL_TO",
                Self::F32 => "F32",
                Self::F64 => "F64",
                Self::GREATER_THAN => "GREATER_THAN",
                Self::GREATER_THAN_OR_EQUAL_TO => "GREATER_THAN_OR_EQUAL_TO",
                Self::I8 => "I8",
                Self::I16 => "I16",
                Self::I32 => "I32",
                Self::I64 => "I64",
                Self::ISIZE => "ISIZE",
                Self::IDENTIFIER => "IDENTIFIER",
                Self::LOWER_THAN => "LOWER_THAN",
                Self::LOWER_THAN_OR_EQUAL_TO => "LOWER_THAN_OR_EQUAL_TO",
                Self::MINUS => "MINUS",
                Self::NEWLINE => "NEWLINE",
                Self::NOT_EQUAL_TO => "NOT_EQUAL_TO",
                Self::OPEN_BRACE => "OPEN_BRACE",
                Self::OPEN_PARENTHESIS => "OPEN_PARENTHESIS",
                Self::PLUS => "PLUS",
                Self::SEMICOLON => "SEMICOLON",
                Self::SLASH => "SLASH",
                Self::U8 => "U8",
                Self::U16 => "U16",
                Self::U32 => "U32",
                Self::U64 => "U64",
                Self::USIZE => "USIZE",
                Self::WHITESPACE => "WHITESPACE",
            }
        ))
    }
}
