use pyo3::exceptions::PyValueError;
use pyo3::pyclass::CompareOp;
use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyObject, PyResult, PyTypeInfo, Python,
};

use az::tokenization::{NumericLiteralType, TokenContent};

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::py_token_kind::PyTokenKind;
use super::traits::{Repr, RichCmp};
use super::types::TokenOwnedStr;

#[derive(Clone, PartialEq)]
#[pyclass(module = "az.tokenization", name = "TokenContent", frozen)]
pub(crate) struct PyTokenContent(TokenContent<TokenOwnedStr>);

#[pymethods]
impl PyTokenContent {
    #[getter]
    fn kind(&self) -> PyTokenKind {
        PyTokenKind::from(&self.0)
    }

    #[getter]
    fn string(&self) -> String {
        self.0.to_string()
    }

    #[new]
    #[pyo3(signature = (kind, string, /))]
    fn new(kind: PyTokenKind, string: String) -> PyResult<Self> {
        Ok(match kind {
            PyTokenKind::ARROW => Self(TokenContent::Arrow),
            PyTokenKind::ASSIGNMENT => Self(TokenContent::Assignment),
            PyTokenKind::ASTERISK => Self(TokenContent::Asterisk),
            PyTokenKind::CLOSE_BRACE => Self(TokenContent::CloseBrace),
            PyTokenKind::CLOSE_PARENTHESIS => {
                Self(TokenContent::CloseParenthesis)
            }
            PyTokenKind::COLON => Self(TokenContent::Colon),
            PyTokenKind::COMMA => Self(TokenContent::Comma),
            PyTokenKind::COMMENT_BLOCK => Self(TokenContent::CommentBlock(
                string.split_inclusive('\n').map(Into::into).collect(),
            )),
            PyTokenKind::COMMENT_LINE => {
                Self(TokenContent::CommentLine(string.into()))
            }
            PyTokenKind::DOT => Self(TokenContent::Dot),
            PyTokenKind::EQUAL_TO => Self(TokenContent::EqualTo),
            PyTokenKind::F32
            | PyTokenKind::F64
            | PyTokenKind::I8
            | PyTokenKind::I16
            | PyTokenKind::I32
            | PyTokenKind::I64
            | PyTokenKind::ISIZE
            | PyTokenKind::U8
            | PyTokenKind::U16
            | PyTokenKind::U32
            | PyTokenKind::U64
            | PyTokenKind::USIZE => {
                let type_ = match kind {
                    PyTokenKind::F32 => NumericLiteralType::F32,
                    PyTokenKind::F64 => NumericLiteralType::F64,
                    PyTokenKind::I8 => NumericLiteralType::I8,
                    PyTokenKind::I16 => NumericLiteralType::I16,
                    PyTokenKind::I32 => NumericLiteralType::I32,
                    PyTokenKind::I64 => NumericLiteralType::I64,
                    PyTokenKind::ISIZE => NumericLiteralType::ISize,
                    PyTokenKind::U8 => NumericLiteralType::U8,
                    PyTokenKind::U16 => NumericLiteralType::U16,
                    PyTokenKind::U32 => NumericLiteralType::U32,
                    PyTokenKind::U64 => NumericLiteralType::U64,
                    PyTokenKind::USIZE => NumericLiteralType::USize,
                    _ => unreachable!(),
                };
                Self(TokenContent::NumericLiteral {
                    value: parse_numeric_literal_value(&string, type_)?.into(),
                    type_,
                })
            }
            PyTokenKind::GREATER_THAN => Self(TokenContent::GreaterThan),
            PyTokenKind::GREATER_THAN_OR_EQUAL_TO => {
                Self(TokenContent::GreaterThanOrEqualTo)
            }
            PyTokenKind::IDENTIFIER => {
                Self(TokenContent::Identifier(string.into()))
            }
            PyTokenKind::LOWER_THAN => Self(TokenContent::LowerThan),
            PyTokenKind::LOWER_THAN_OR_EQUAL_TO => {
                Self(TokenContent::LowerThanOrEqualTo)
            }
            PyTokenKind::MINUS => Self(TokenContent::Minus),
            PyTokenKind::NEWLINE => Self(TokenContent::Newline),
            PyTokenKind::NOT_EQUAL_TO => Self(TokenContent::NotEqualTo),
            PyTokenKind::OPEN_BRACE => Self(TokenContent::OpenBrace),
            PyTokenKind::OPEN_PARENTHESIS => {
                Self(TokenContent::OpenParenthesis)
            }
            PyTokenKind::PLUS => Self(TokenContent::Plus),
            PyTokenKind::SEMICOLON => Self(TokenContent::Semicolon),
            PyTokenKind::SLASH => Self(TokenContent::Slash),
            PyTokenKind::WHITESPACE => {
                Self(TokenContent::Whitespace(string.into()))
            }
        })
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        self.rich_cmp(other, op, py)
    }
}

impl From<TokenContent<TokenOwnedStr>> for PyTokenContent {
    fn from(value: TokenContent<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyTokenContent> for TokenContent<TokenOwnedStr> {
    fn from(value: PyTokenContent) -> Self {
        value.0
    }
}

impl Repr for TokenContent<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            PyTokenContent::NAME,
            PyTokenKind::from(self).repr(py)?,
            self.to_string().repr(py)?
        ))
    }
}

impl Repr for PyTokenContent {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyTokenContent);

fn parse_numeric_literal_value(
    string: &String,
    type_: NumericLiteralType,
) -> PyResult<&str> {
    string
        .rsplit_once('_')
        .and_then(|(value, suffix)| {
            if !value.is_empty() && suffix.eq(type_.to_string().as_str()) {
                Some(value)
            } else {
                None
            }
        })
        .ok_or_else(|| {
            PyValueError::new_err(format!(
                "Invalid numeric literal string: {}.",
                string
            ))
        })
}
