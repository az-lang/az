use pyo3::exceptions::PyException;
use pyo3::{pyclass, PyClassInitializer};

#[pyclass(
    module = "az.parsing", name = "ParsingError", extends = PyException,
    frozen, subclass
)]
pub(crate) struct PyParsingError {}

impl PyParsingError {
    pub(super) fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Self {})
    }
}
