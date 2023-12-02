use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use az::tokenization::NumericLiteralValueKind;

use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone, Eq, PartialEq)]
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

impl From<PyNumericLiteralValueKind> for NumericLiteralValueKind {
    fn from(value: PyNumericLiteralValueKind) -> Self {
        match value {
            PyNumericLiteralValueKind::FLOATING_POINT => {
                NumericLiteralValueKind::FloatingPoint
            }
            PyNumericLiteralValueKind::INTEGER => {
                NumericLiteralValueKind::Integer
            }
        }
    }
}

impl Repr for NumericLiteralValueKind {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            PyNumericLiteralValueKind::NAME,
            match self {
                Self::FloatingPoint => "FLOATING_POINT",
                Self::Integer => "INTEGER",
            }
        ))
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
