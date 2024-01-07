use az::tokenization::NumericLiteralValueKind;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[pyclass(
    module = "az.tokenization",
    name = "NumericLiteralValueKind",
    frozen
)]
pub(super) enum PyNumericLiteralValueKind {
    FLOATING_POINT,
    INTEGER,
}

#[pymethods]
impl PyNumericLiteralValueKind {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<NumericLiteralValueKind> for PyNumericLiteralValueKind {
    fn from(value: NumericLiteralValueKind) -> Self {
        match value {
            NumericLiteralValueKind::FloatingPoint => Self::FLOATING_POINT,
            NumericLiteralValueKind::Integer => Self::INTEGER,
        }
    }
}

impl Repr for PyNumericLiteralValueKind {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self {
                Self::FLOATING_POINT => "FLOATING_POINT",
                Self::INTEGER => "INTEGER",
            }
        ))
    }
}
