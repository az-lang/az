use pyo3::{pyclass, PyClassInitializer};

#[pyclass(module = "az.parsing", name = "Expression", frozen, subclass)]
pub(crate) struct PyExpression {}

impl PyExpression {
    pub(super) fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Self {})
    }
}
