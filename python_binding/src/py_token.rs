use az::tokenization::{NumericLiteralType, Token};
use pyo3::types::PyString;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::py_token_kind::PyTokenKind;
use super::traits::Repr;

#[derive(Clone)]
#[pyclass(module = "az.tokenization", name = "Token")]
pub(super) struct PyToken {
    #[pyo3(get)]
    kind: PyTokenKind,
    #[pyo3(get)]
    string: String,
}

#[pymethods]
impl PyToken {
    #[new]
    fn new(kind: PyTokenKind, string: String) -> Self {
        Self { kind, string }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl<'a> From<Token<'a>> for PyToken {
    fn from(value: Token<'a>) -> Self {
        let kind = {
            match value {
                Token::Arrow => PyTokenKind::ARROW,
                Token::Assignment => PyTokenKind::ASSIGNMENT,
                Token::Asterisk => PyTokenKind::ASTERISK,
                Token::CloseBrace => PyTokenKind::CLOSE_BRACE,
                Token::CloseParenthesis => PyTokenKind::CLOSE_PARENTHESIS,
                Token::Colon => PyTokenKind::COLON,
                Token::Comma => PyTokenKind::COMMA,
                Token::CommentBlock(_) => PyTokenKind::COMMENT_BLOCK,
                Token::CommentLine(_) => PyTokenKind::COMMENT_LINE,
                Token::Dot => PyTokenKind::DOT,
                Token::EqualTo => PyTokenKind::EQUAL_TO,
                Token::GreaterThan => PyTokenKind::GREATER_THAN,
                Token::GreaterThanOrEqualTo => {
                    PyTokenKind::GREATER_THAN_OR_EQUAL_TO
                }
                Token::Identifier(_) => PyTokenKind::IDENTIFIER,
                Token::LowerThan => PyTokenKind::LOWER_THAN,
                Token::LowerThanOrEqualTo => {
                    PyTokenKind::LOWER_THAN_OR_EQUAL_TO
                }
                Token::Minus => PyTokenKind::MINUS,
                Token::Newline => PyTokenKind::NEWLINE,
                Token::NotEqualTo => PyTokenKind::NOT_EQUAL_TO,
                Token::NumericLiteral { ref type_, .. } => match type_ {
                    NumericLiteralType::F32 => PyTokenKind::F32,
                    NumericLiteralType::F64 => PyTokenKind::F64,
                    NumericLiteralType::I8 => PyTokenKind::I8,
                    NumericLiteralType::I16 => PyTokenKind::I16,
                    NumericLiteralType::I32 => PyTokenKind::I32,
                    NumericLiteralType::I64 => PyTokenKind::I64,
                    NumericLiteralType::ISize => PyTokenKind::ISIZE,
                    NumericLiteralType::U8 => PyTokenKind::U8,
                    NumericLiteralType::U16 => PyTokenKind::U16,
                    NumericLiteralType::U32 => PyTokenKind::U32,
                    NumericLiteralType::U64 => PyTokenKind::U64,
                    NumericLiteralType::USize => PyTokenKind::USIZE,
                },
                Token::OpenBrace => PyTokenKind::OPEN_BRACE,
                Token::OpenParenthesis => PyTokenKind::OPEN_PARENTHESIS,
                Token::Plus => PyTokenKind::PLUS,
                Token::Semicolon => PyTokenKind::SEMICOLON,
                Token::Slash => PyTokenKind::SLASH,
                Token::Whitespace(_) => PyTokenKind::WHITESPACE,
            }
        };
        Self {
            kind,
            string: value.to_string(),
        }
    }
}

impl<'a> From<&'a PyToken> for Token<'a> {
    fn from(value: &'a PyToken) -> Self {
        match value.kind {
            PyTokenKind::ARROW => Self::Arrow,
            PyTokenKind::ASSIGNMENT => Self::Assignment,
            PyTokenKind::ASTERISK => Self::Asterisk,
            PyTokenKind::CLOSE_BRACE => Self::CloseBrace,
            PyTokenKind::CLOSE_PARENTHESIS => Self::CloseParenthesis,
            PyTokenKind::COLON => Self::Colon,
            PyTokenKind::COMMA => Self::Comma,
            PyTokenKind::COMMENT_BLOCK => {
                Self::CommentBlock(value.string.lines().collect())
            }
            PyTokenKind::COMMENT_LINE => {
                Self::CommentLine(value.string.as_str())
            }
            PyTokenKind::DOT => Self::Dot,
            PyTokenKind::EQUAL_TO => Self::EqualTo,
            PyTokenKind::F32 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::F32,
            },
            PyTokenKind::F64 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::F64,
            },
            PyTokenKind::GREATER_THAN => Self::GreaterThan,
            PyTokenKind::GREATER_THAN_OR_EQUAL_TO => {
                Self::GreaterThanOrEqualTo
            }
            PyTokenKind::I8 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::I8,
            },
            PyTokenKind::I16 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::I16,
            },
            PyTokenKind::I32 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::I32,
            },
            PyTokenKind::I64 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::I64,
            },
            PyTokenKind::IDENTIFIER => Self::Identifier(value.string.as_str()),
            PyTokenKind::ISIZE => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::ISize,
            },
            PyTokenKind::LOWER_THAN => Self::LowerThan,
            PyTokenKind::LOWER_THAN_OR_EQUAL_TO => Self::LowerThanOrEqualTo,
            PyTokenKind::MINUS => Self::Minus,
            PyTokenKind::NEWLINE => Self::Newline,
            PyTokenKind::NOT_EQUAL_TO => Self::NotEqualTo,
            PyTokenKind::OPEN_BRACE => Self::OpenBrace,
            PyTokenKind::OPEN_PARENTHESIS => Self::OpenParenthesis,
            PyTokenKind::PLUS => Self::Plus,
            PyTokenKind::SEMICOLON => Self::Semicolon,
            PyTokenKind::SLASH => Self::Slash,
            PyTokenKind::U8 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::U8,
            },
            PyTokenKind::U16 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::U16,
            },
            PyTokenKind::U32 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::U32,
            },
            PyTokenKind::U64 => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::U64,
            },
            PyTokenKind::USIZE => Self::NumericLiteral {
                value: value.string.as_str(),
                type_: NumericLiteralType::USize,
            },
            PyTokenKind::WHITESPACE => Self::Whitespace(value.string.as_str()),
        }
    }
}

impl Repr for PyToken {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.kind.repr(py)?,
            PyString::new(py, self.string.as_str()).repr()?
        ))
    }
}
