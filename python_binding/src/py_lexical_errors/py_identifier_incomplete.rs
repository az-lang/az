use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::tokenization::IdentifierIncomplete;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedStr;

use super::py_lexical_error::PyLexicalError;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.tokenization", name = "IdentifierIncomplete",
    extends = PyLexicalError, frozen
)]
pub(crate) struct PyIdentifierIncomplete(IdentifierIncomplete<TokenOwnedStr>);

#[pymethods]
impl PyIdentifierIncomplete {
    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[getter]
    fn string(&self) -> String {
        self.0.string.as_ref().into()
    }

    #[new]
    #[pyo3(signature = (position, string, /))]
    fn new(
        position: PySubstringPosition,
        string: String,
    ) -> PyClassInitializer<Self> {
        PyLexicalError::new().add_subclass(Self(IdentifierIncomplete {
            position: position.into(),
            string: string.into(),
        }))
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: pyclass::CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        self.rich_cmp(other, op, py)
    }
}

impl From<IdentifierIncomplete<TokenOwnedStr>> for PyIdentifierIncomplete {
    fn from(value: IdentifierIncomplete<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyIdentifierIncomplete> for PyErr {
    fn from(value: PyIdentifierIncomplete) -> Self {
        Self::new::<PyIdentifierIncomplete, _>((
            value.position(),
            value.string(),
        ))
    }
}

impl Repr for IdentifierIncomplete<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            PyIdentifierIncomplete::NAME,
            self.string.repr(py)?,
            self.position.repr(py)?
        ))
    }
}

impl Repr for PyIdentifierIncomplete {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyIdentifierIncomplete,
    PyLexicalError
);
