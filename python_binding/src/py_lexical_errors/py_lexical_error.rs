use pyo3::exceptions::PyException;
use pyo3::{pyclass, PyClassInitializer};

#[pyclass(
    module = "az.tokenization", name = "LexicalError", extends = PyException,
    frozen, subclass
)]
pub struct PyLexicalError {}

impl PyLexicalError {
    pub(super) fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Self {})
    }
}
