use pyo3::{pyclass, PyClassInitializer};

#[pyclass(module = "az.parsing", name = "Statement", frozen, subclass)]
pub(crate) struct PyStatement {}

impl PyStatement {
    pub(super) fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Self {})
    }
}
