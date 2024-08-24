use pyo3::exceptions::PyException;
use pyo3::{pyclass, PyClassInitializer};

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.tokenization", name = "ValidationError",
    extends = PyException, frozen, subclass
)]
pub(crate) struct PyValidationError;

impl PyValidationError {
    pub(crate) fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Self)
    }
}
