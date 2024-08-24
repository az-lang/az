use pyo3::pyclass::CompareOp;
use pyo3::types::PyType;
use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyObject, PyRef, PyResult, PyTypeInfo,
    Python,
};

use az::tokenization::TokenCollection;

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::py_lexical_errors::OwnedLexicalErrorWrapper;
use super::py_token::PyToken;
use super::traits::{Repr, RichCmp};
use super::types::TokenOwnedString;
use super::validation::{validate_contents, validate_positions};

#[derive(Clone, PartialEq)]
#[pyclass(module = "az.tokenization", name = "TokenCollection", frozen)]
pub(crate) struct PyTokenCollection(TokenCollection<TokenOwnedString>);

#[pymethods]
impl PyTokenCollection {
    #[classmethod]
    #[pyo3(signature = (string, /))]
    fn from_string(_cls: &Bound<'_, PyType>, string: &str) -> PyResult<Self> {
        TokenCollection::try_from(string)
            .map(Self)
            .map_err(|error| OwnedLexicalErrorWrapper::from(error).into())
    }

    #[new]
    #[pyo3(signature = (_tokens, /))]
    fn new(_tokens: Vec<PyToken>) -> Self {
        Self(TokenCollection::new(
            _tokens.into_iter().map(PyToken::into).collect(),
        ))
    }

    fn validate_contents(&self, py: Python<'_>) -> PyResult<()> {
        validate_contents(&self.0, py)
    }

    fn validate_positions(&self, py: Python<'_>) -> PyResult<()> {
        validate_positions(&self.0, py)
    }

    fn __iter__(&self) -> PyTokenIterator {
        PyTokenIterator(self.0.clone().into_iter())
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

#[pyclass(module = "az.tokenization", name = "_TokenIterator")]
struct PyTokenIterator(
    <TokenCollection<TokenOwnedString> as IntoIterator>::IntoIter,
);

#[pymethods]
impl PyTokenIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyToken> {
        self.0.next().map(PyToken::from)
    }
}

impl From<PyTokenCollection> for TokenCollection<TokenOwnedString> {
    fn from(value: PyTokenCollection) -> Self {
        value.0
    }
}

impl From<TokenCollection<TokenOwnedString>> for PyTokenCollection {
    fn from(value: TokenCollection<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl Repr for PyTokenCollection {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl Repr for TokenCollection<TokenOwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            PyTokenCollection::NAME,
            format_args!(
                "[{}]",
                self.iter()
                    .map(|token| token.repr(py))
                    .collect::<PyResult<Vec<_>>>()?
                    .join(", ")
            )
        ))
    }
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyTokenCollection);
