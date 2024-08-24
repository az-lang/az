use pyo3::exceptions::{PyBaseExceptionGroup, PyValueError};
use pyo3::pyclass::CompareOp;
use pyo3::{
    pyclass, pymethods, Bound, Py, PyAny, PyErr, PyObject, PyResult,
    PyTypeInfo, Python,
};

use az::tokenization::{NumericLiteralType, TokenContent};

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::py_token_kind::{PyTokenKind, TokenKind};
use super::py_validation_errors::PyContentsValidationError;
use super::traits::{Repr, RichCmp};
use super::types::TokenOwnedString;

#[derive(Clone, PartialEq)]
#[pyclass(module = "az.tokenization", name = "TokenContent", frozen)]
pub(crate) struct PyTokenContent(TokenContent<TokenOwnedString>);

#[pymethods]
impl PyTokenContent {
    #[new]
    #[pyo3(signature = (kind, state=None, /))]
    fn new(
        kind: PyTokenKind,
        state: Option<String>,
        py: Python<'_>,
    ) -> PyResult<Self> {
        match (kind.into(), state) {
            (TokenKind::Arrow, None) => Ok(Self(TokenContent::Arrow)),
            (TokenKind::Assignment, None) => {
                Ok(Self(TokenContent::Assignment))
            }
            (TokenKind::Asterisk, None) => Ok(Self(TokenContent::Asterisk)),
            (TokenKind::CloseBrace, None) => {
                Ok(Self(TokenContent::CloseBrace))
            }
            (TokenKind::CloseParenthesis, None) => {
                Ok(Self(TokenContent::CloseParenthesis))
            }
            (TokenKind::Colon, None) => Ok(Self(TokenContent::Colon)),
            (TokenKind::Comma, None) => Ok(Self(TokenContent::Comma)),
            (TokenKind::CommentBlock, Some(state)) => {
                Ok(Self(TokenContent::CommentBlock(state.into())))
            }
            (TokenKind::CommentLine, Some(state)) => {
                Ok(Self(TokenContent::CommentLine(state.into())))
            }
            (TokenKind::Dot, None) => Ok(Self(TokenContent::Dot)),
            (TokenKind::EqualTo, None) => Ok(Self(TokenContent::EqualTo)),
            (
                kind @ (TokenKind::F32
                | TokenKind::F64
                | TokenKind::I8
                | TokenKind::I16
                | TokenKind::I32
                | TokenKind::I64
                | TokenKind::ISize
                | TokenKind::U8
                | TokenKind::U16
                | TokenKind::U32
                | TokenKind::U64
                | TokenKind::USize),
                Some(state),
            ) => {
                let type_ = match kind {
                    TokenKind::F32 => NumericLiteralType::F32,
                    TokenKind::F64 => NumericLiteralType::F64,
                    TokenKind::I8 => NumericLiteralType::I8,
                    TokenKind::I16 => NumericLiteralType::I16,
                    TokenKind::I32 => NumericLiteralType::I32,
                    TokenKind::I64 => NumericLiteralType::I64,
                    TokenKind::ISize => NumericLiteralType::ISize,
                    TokenKind::U8 => NumericLiteralType::U8,
                    TokenKind::U16 => NumericLiteralType::U16,
                    TokenKind::U32 => NumericLiteralType::U32,
                    TokenKind::U64 => NumericLiteralType::U64,
                    TokenKind::USize => NumericLiteralType::USize,
                    _ => unreachable!(),
                };
                Ok(Self(TokenContent::NumericLiteral {
                    value: state.into(),
                    type_,
                }))
            }
            (TokenKind::GreaterThan, None) => {
                Ok(Self(TokenContent::GreaterThan))
            }
            (TokenKind::GreaterThanOrEqualTo, None) => {
                Ok(Self(TokenContent::GreaterThanOrEqualTo))
            }
            (TokenKind::Identifier, Some(state)) => {
                Ok(Self(TokenContent::Identifier(state.into())))
            }
            (TokenKind::LessThan, None) => Ok(Self(TokenContent::LessThan)),
            (TokenKind::LessThanOrEqualTo, None) => {
                Ok(Self(TokenContent::LessThanOrEqualTo))
            }
            (TokenKind::Minus, None) => Ok(Self(TokenContent::Minus)),
            (TokenKind::Newline, None) => Ok(Self(TokenContent::Newline)),
            (TokenKind::NotEqualTo, None) => {
                Ok(Self(TokenContent::NotEqualTo))
            }
            (TokenKind::OpenBrace, None) => Ok(Self(TokenContent::OpenBrace)),
            (TokenKind::OpenParenthesis, None) => {
                Ok(Self(TokenContent::OpenParenthesis))
            }
            (TokenKind::Plus, None) => Ok(Self(TokenContent::Plus)),
            (TokenKind::Semicolon, None) => Ok(Self(TokenContent::Semicolon)),
            (TokenKind::Slash, None) => Ok(Self(TokenContent::Slash)),
            (TokenKind::Whitespace, Some(state)) => {
                Ok(Self(TokenContent::Whitespace(state.into())))
            }
            (kind, state) => Err(PyValueError::new_err(format!(
                "Invalid arguments for {} with {} kind: {}, but got {}.",
                Self::NAME,
                kind.repr(py)?,
                match state {
                    Some(_) => "does not need a state",
                    None => "needs a state",
                },
                state.repr(py)?
            ))),
        }
    }

    #[getter]
    fn kind(&self, py: Python<'_>) -> Py<PyTokenKind> {
        PyTokenKind::from_rust(TokenKind::from(&self.0), py)
    }

    #[getter]
    fn state(&self) -> Option<String> {
        match &self.0 {
            TokenContent::Arrow
            | TokenContent::Assignment
            | TokenContent::Asterisk
            | TokenContent::CloseBrace
            | TokenContent::CloseParenthesis
            | TokenContent::Colon
            | TokenContent::Comma
            | TokenContent::Dot
            | TokenContent::EqualTo
            | TokenContent::GreaterThan
            | TokenContent::GreaterThanOrEqualTo
            | TokenContent::LessThan
            | TokenContent::LessThanOrEqualTo
            | TokenContent::Minus
            | TokenContent::Newline
            | TokenContent::NotEqualTo
            | TokenContent::OpenBrace
            | TokenContent::OpenParenthesis
            | TokenContent::Plus
            | TokenContent::Semicolon
            | TokenContent::Slash => None,
            TokenContent::CommentBlock(_)
            | TokenContent::CommentLine(_)
            | TokenContent::Identifier(_)
            | TokenContent::Whitespace(_) => Some(self.0.to_string()),
            TokenContent::NumericLiteral { value, .. } => {
                Some(value.clone().into())
            }
        }
    }

    fn validate(&self) -> PyResult<()> {
        self.0.validate().map_err(|errors| {
            PyBaseExceptionGroup::new_err((
                format!("Failed to validate {}.", PyTokenContent::NAME),
                errors
                    .into_iter()
                    .map(|error| {
                        PyErr::new::<PyContentsValidationError, _>((
                            error.to_string(),
                        ))
                    })
                    .collect::<Vec<_>>(),
            ))
        })
    }

    fn __str__(&self) -> String {
        self.0.to_string()
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

impl From<TokenContent<TokenOwnedString>> for PyTokenContent {
    fn from(value: TokenContent<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyTokenContent> for TokenContent<TokenOwnedString> {
    fn from(value: PyTokenContent) -> Self {
        value.0
    }
}

impl Repr for TokenContent<TokenOwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(match self {
            TokenContent::Arrow
            | TokenContent::Assignment
            | TokenContent::Asterisk
            | TokenContent::CloseBrace
            | TokenContent::CloseParenthesis
            | TokenContent::Colon
            | TokenContent::Comma
            | TokenContent::Dot
            | TokenContent::EqualTo
            | TokenContent::GreaterThan
            | TokenContent::GreaterThanOrEqualTo
            | TokenContent::LessThan
            | TokenContent::LessThanOrEqualTo
            | TokenContent::Minus
            | TokenContent::Newline
            | TokenContent::NotEqualTo
            | TokenContent::OpenBrace
            | TokenContent::OpenParenthesis
            | TokenContent::Plus
            | TokenContent::Semicolon
            | TokenContent::Slash => {
                format!(
                    "{}({})",
                    PyTokenContent::NAME,
                    TokenKind::from(self).repr(py)?
                )
            }
            TokenContent::CommentBlock(_)
            | TokenContent::CommentLine(_)
            | TokenContent::Identifier(_)
            | TokenContent::Whitespace(_) => format!(
                "{}({}, {})",
                PyTokenContent::NAME,
                TokenKind::from(self).repr(py)?,
                self.to_string().repr(py)?
            ),
            TokenContent::NumericLiteral { value, .. } => format!(
                "{}({}, {})",
                PyTokenContent::NAME,
                TokenKind::from(self).repr(py)?,
                value.repr(py)?
            ),
        })
    }
}

impl Repr for PyTokenContent {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyTokenContent);
