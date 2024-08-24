use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::parsing::OutOfTokens;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::traits::{Repr, RichCmp};

use super::py_parsing_error::PyParsingError;

#[derive(Clone, PartialEq)]
#[pyclass(
module = "az.parsing", name = "OutOfTokens",
extends = PyParsingError, frozen
)]
pub(crate) struct PyOutOfTokens(OutOfTokens);

#[pymethods]
impl PyOutOfTokens {
    #[new]
    fn new() -> PyClassInitializer<Self> {
        PyParsingError::new().add_subclass(Self(OutOfTokens))
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

impl From<OutOfTokens> for PyOutOfTokens {
    fn from(value: OutOfTokens) -> Self {
        Self(value)
    }
}

impl From<PyOutOfTokens> for PyErr {
    fn from(_value: PyOutOfTokens) -> Self {
        Self::new::<PyOutOfTokens, _>(())
    }
}

impl Repr for OutOfTokens {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!("{}()", PyOutOfTokens::NAME))
    }
}

impl Repr for PyOutOfTokens {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyOutOfTokens, PyParsingError);
