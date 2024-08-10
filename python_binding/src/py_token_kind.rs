use pyo3::sync::GILOnceCell;
use pyo3::{pyclass, pymethods, Bound, Py, PyResult, PyTypeInfo, Python};

use az::tokenization::{NumericLiteralType, TokenContent};

use super::traits::Repr;

#[derive(Clone, Debug)]
pub(crate) enum TokenKind {
    Arrow,
    Assignment,
    Asterisk,
    CloseBrace,
    CloseParenthesis,
    Colon,
    Comma,
    CommentBlock,
    CommentLine,
    Dot,
    EqualTo,
    F32,
    F64,
    GreaterThan,
    GreaterThanOrEqualTo,
    I8,
    I16,
    I32,
    I64,
    Identifier,
    ISize,
    LessThan,
    LessThanOrEqualTo,
    Minus,
    Newline,
    NotEqualTo,
    OpenBrace,
    OpenParenthesis,
    Plus,
    Semicolon,
    Slash,
    U8,
    U16,
    U32,
    U64,
    USize,
    Whitespace,
}

impl<StringType> From<&TokenContent<StringType>> for TokenKind {
    fn from(value: &TokenContent<StringType>) -> Self {
        match value {
            TokenContent::Arrow => Self::Arrow,
            TokenContent::Assignment => Self::Assignment,
            TokenContent::Asterisk => Self::Asterisk,
            TokenContent::CloseBrace => Self::CloseBrace,
            TokenContent::CloseParenthesis => Self::CloseParenthesis,
            TokenContent::Colon => Self::Colon,
            TokenContent::Comma => Self::Comma,
            TokenContent::CommentBlock(_) => Self::CommentBlock,
            TokenContent::CommentLine(_) => Self::CommentLine,
            TokenContent::Dot => Self::Dot,
            TokenContent::EqualTo => Self::EqualTo,
            TokenContent::GreaterThan => Self::GreaterThan,
            TokenContent::GreaterThanOrEqualTo => Self::GreaterThanOrEqualTo,
            TokenContent::NumericLiteral { type_, .. } => match type_ {
                NumericLiteralType::F32 => Self::F32,
                NumericLiteralType::F64 => Self::F64,
                NumericLiteralType::I8 => Self::I8,
                NumericLiteralType::I16 => Self::I16,
                NumericLiteralType::I32 => Self::I32,
                NumericLiteralType::I64 => Self::I64,
                NumericLiteralType::ISize => Self::ISize,
                NumericLiteralType::U8 => Self::U8,
                NumericLiteralType::U16 => Self::U16,
                NumericLiteralType::U32 => Self::U32,
                NumericLiteralType::U64 => Self::U64,
                NumericLiteralType::USize => Self::USize,
            },
            TokenContent::Identifier(_) => Self::Identifier,
            TokenContent::LessThan => Self::LessThan,
            TokenContent::LessThanOrEqualTo => Self::LessThanOrEqualTo,
            TokenContent::Minus => Self::Minus,
            TokenContent::Newline => Self::Newline,
            TokenContent::NotEqualTo => Self::NotEqualTo,
            TokenContent::OpenBrace => Self::OpenBrace,
            TokenContent::OpenParenthesis => Self::OpenParenthesis,
            TokenContent::Plus => Self::Plus,
            TokenContent::Semicolon => Self::Semicolon,
            TokenContent::Slash => Self::Slash,
            TokenContent::Whitespace(_) => Self::Whitespace,
        }
    }
}

impl From<PyTokenKind> for TokenKind {
    fn from(value: PyTokenKind) -> Self {
        value.0
    }
}

#[derive(Clone)]
#[pyclass(module = "az.tokenization", name = "TokenKind", frozen)]
pub(crate) struct PyTokenKind(TokenKind);

impl PyTokenKind {
    pub(crate) fn from_rust(value: TokenKind, py: Python<'_>) -> Py<Self> {
        const RUST_VALUES: [TokenKind; 37usize] = [
            TokenKind::Arrow,
            TokenKind::Assignment,
            TokenKind::Asterisk,
            TokenKind::CloseBrace,
            TokenKind::CloseParenthesis,
            TokenKind::Colon,
            TokenKind::Comma,
            TokenKind::CommentBlock,
            TokenKind::CommentLine,
            TokenKind::Dot,
            TokenKind::EqualTo,
            TokenKind::F32,
            TokenKind::F64,
            TokenKind::GreaterThan,
            TokenKind::GreaterThanOrEqualTo,
            TokenKind::I8,
            TokenKind::I16,
            TokenKind::I32,
            TokenKind::I64,
            TokenKind::Identifier,
            TokenKind::ISize,
            TokenKind::LessThan,
            TokenKind::LessThanOrEqualTo,
            TokenKind::Minus,
            TokenKind::Newline,
            TokenKind::NotEqualTo,
            TokenKind::OpenBrace,
            TokenKind::OpenParenthesis,
            TokenKind::Plus,
            TokenKind::Semicolon,
            TokenKind::Slash,
            TokenKind::U8,
            TokenKind::U16,
            TokenKind::U32,
            TokenKind::U64,
            TokenKind::USize,
            TokenKind::Whitespace,
        ];
        static PY_VALUES: GILOnceCell<[Py<PyTokenKind>; RUST_VALUES.len()]> =
            GILOnceCell::new();
        PY_VALUES.get_or_init(py, || {
            RUST_VALUES
                .map(|value| Bound::new(py, Self(value)).unwrap().into())
        })[RUST_VALUES
            .iter()
            .position(|candidate| {
                std::mem::discriminant(candidate)
                    == std::mem::discriminant(&value)
            })
            .unwrap()]
        .clone_ref(py)
    }
}

#[pymethods]
impl PyTokenKind {
    #[allow(non_snake_case)]
    #[classattr]
    fn ARROW(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Arrow, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ASSIGNMENT(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Assignment, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ASTERISK(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Asterisk, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn CLOSE_BRACE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::CloseBrace, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn CLOSE_PARENTHESIS(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::CloseParenthesis, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn COLON(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Colon, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn COMMA(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Comma, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn COMMENT_BLOCK(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::CommentBlock, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn COMMENT_LINE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::CommentLine, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn DOT(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Dot, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn EQUAL_TO(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::EqualTo, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn F32(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::F32, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn F64(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::F64, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn GREATER_THAN(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::GreaterThan, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn GREATER_THAN_OR_EQUAL_TO(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::GreaterThanOrEqualTo, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn I8(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::I8, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn I16(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::I16, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn I32(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::I32, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn I64(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::I64, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn IDENTIFIER(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Identifier, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ISIZE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::ISize, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn LESS_THAN(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::LessThan, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn LESS_THAN_OR_EQUAL_TO(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::LessThanOrEqualTo, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MINUS(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Minus, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NEWLINE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Newline, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NOT_EQUAL_TO(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::NotEqualTo, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn OPEN_BRACE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::OpenBrace, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn OPEN_PARENTHESIS(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::OpenParenthesis, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PLUS(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Plus, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn SEMICOLON(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Semicolon, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn SLASH(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Slash, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn U8(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::U8, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn U16(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::U16, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn U32(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::U32, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn U64(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::U64, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn USIZE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::USize, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn WHITESPACE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(TokenKind::Whitespace, py)
    }

    fn __reduce__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl Repr for PyTokenKind {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl Repr for TokenKind {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            PyTokenKind::NAME,
            match self {
                Self::Arrow => "ARROW",
                Self::Assignment => "ASSIGNMENT",
                Self::Asterisk => "ASTERISK",
                Self::CloseBrace => "CLOSE_BRACE",
                Self::CloseParenthesis => "CLOSE_PARENTHESIS",
                Self::Colon => "COLON",
                Self::Comma => "COMMA",
                Self::CommentBlock => "COMMENT_BLOCK",
                Self::CommentLine => "COMMENT_LINE",
                Self::Dot => "DOT",
                Self::EqualTo => "EQUAL_TO",
                Self::F32 => "F32",
                Self::F64 => "F64",
                Self::GreaterThan => "GREATER_THAN",
                Self::GreaterThanOrEqualTo => "GREATER_THAN_OR_EQUAL_TO",
                Self::I8 => "I8",
                Self::I16 => "I16",
                Self::I32 => "I32",
                Self::I64 => "I64",
                Self::ISize => "ISIZE",
                Self::Identifier => "IDENTIFIER",
                Self::LessThan => "LESS_THAN",
                Self::LessThanOrEqualTo => "LESS_THAN_OR_EQUAL_TO",
                Self::Minus => "MINUS",
                Self::Newline => "NEWLINE",
                Self::NotEqualTo => "NOT_EQUAL_TO",
                Self::OpenBrace => "OPEN_BRACE",
                Self::OpenParenthesis => "OPEN_PARENTHESIS",
                Self::Plus => "PLUS",
                Self::Semicolon => "SEMICOLON",
                Self::Slash => "SLASH",
                Self::U8 => "U8",
                Self::U16 => "U16",
                Self::U32 => "U32",
                Self::U64 => "U64",
                Self::USize => "USIZE",
                Self::Whitespace => "WHITESPACE",
            }
        ))
    }
}
