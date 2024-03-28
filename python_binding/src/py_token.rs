use pyo3::pyclass::CompareOp;
use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyObject, PyResult, PyTypeInfo, Python,
};

use az::tokenization::Token;

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::py_substring_position::PySubstringPosition;
use super::py_token_content::PyTokenContent;
use super::traits::{Repr, RichCmp};
use super::types::TokenOwnedStr;

#[derive(Clone, PartialEq)]
#[pyclass(module = "az.tokenization", name = "Token", frozen)]
pub(crate) struct PyToken(Token<TokenOwnedStr>);

#[pymethods]
impl PyToken {
    #[getter]
    fn content(&self) -> PyTokenContent {
        self.0.content.clone().into()
    }

    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[new]
    #[pyo3(signature = (*, content, position))]
    fn new(content: PyTokenContent, position: PySubstringPosition) -> Self {
        Self(Token {
            content: content.into(),
            position: position.into(),
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

impl From<PyToken> for Token<TokenOwnedStr> {
    fn from(value: PyToken) -> Self {
        value.0
    }
}

impl From<Token<TokenOwnedStr>> for PyToken {
    fn from(value: Token<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl Repr for PyToken {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl Repr for Token<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}(content={}, position={})",
            PyToken::NAME,
            self.content.repr(py)?,
            self.position.repr(py)?
        ))
    }
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyToken);
