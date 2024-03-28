use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::parsing::MissingSemicolon;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_token::PyToken;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedStr;

use super::py_parsing_error::PyParsingError;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "MissingSemicolon", extends = PyParsingError,
    frozen
)]
pub(crate) struct PyMissingSemicolon(MissingSemicolon<TokenOwnedStr>);

#[pymethods]
impl PyMissingSemicolon {
    #[getter]
    fn token(&self) -> PyToken {
        self.0.token.clone().into()
    }

    #[new]
    #[pyo3(signature = (token, /))]
    fn new(token: PyToken) -> PyClassInitializer<Self> {
        PyParsingError::new().add_subclass(Self(MissingSemicolon {
            token: token.into(),
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

impl From<MissingSemicolon<TokenOwnedStr>> for PyMissingSemicolon {
    fn from(value: MissingSemicolon<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyMissingSemicolon> for PyErr {
    fn from(value: PyMissingSemicolon) -> Self {
        Self::new::<PyMissingSemicolon, _>((value.token(),))
    }
}

impl Repr for MissingSemicolon<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            PyMissingSemicolon::NAME,
            self.token.repr(py)?
        ))
    }
}

impl Repr for PyMissingSemicolon {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyMissingSemicolon,
    PyParsingError
);
