use az::parsing::UnaryArithmeticOperator;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "UnaryArithmeticOperator", frozen)]
pub(crate) enum PyUnaryArithmeticOperator {
    NEGATION,
}

#[pymethods]
impl PyUnaryArithmeticOperator {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<UnaryArithmeticOperator> for PyUnaryArithmeticOperator {
    fn from(value: UnaryArithmeticOperator) -> Self {
        match value {
            UnaryArithmeticOperator::Negation => Self::NEGATION,
        }
    }
}

impl Repr for PyUnaryArithmeticOperator {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self {
                Self::NEGATION => "NEGATION",
            }
        ))
    }
}
