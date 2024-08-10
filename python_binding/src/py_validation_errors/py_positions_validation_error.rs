use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use crate::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use crate::traits::{Repr, RichCmp};

use super::py_validation_error::PyValidationError;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.tokenization", name = "PositionsValidationError",
    extends = PyValidationError, frozen
)]
pub(crate) struct PyPositionsValidationError {
    message: String,
}

#[pymethods]
impl PyPositionsValidationError {
    #[getter]
    fn message(&self) -> &str {
        &self.message
    }

    #[new]
    #[pyo3(signature = (message, /))]
    pub(crate) fn new(message: String) -> PyClassInitializer<Self> {
        PyValidationError::new().add_subclass(Self { message })
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

impl From<PyPositionsValidationError> for PyErr {
    fn from(value: PyPositionsValidationError) -> Self {
        Self::new::<PyPositionsValidationError, _>((value.message,))
    }
}

impl Repr for PyPositionsValidationError {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!("{}({})", Self::NAME, self.message.repr(py)?))
    }
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyPositionsValidationError);
