use az::parsing::BinaryArithmeticOperator;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "BinaryArithmeticOperator", frozen)]
pub(crate) enum PyBinaryArithmeticOperator {
    ADDITION,
    DIVISION,
    MULTIPLICATION,
    SUBTRACTION,
}

#[pymethods]
impl PyBinaryArithmeticOperator {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<BinaryArithmeticOperator> for PyBinaryArithmeticOperator {
    fn from(value: BinaryArithmeticOperator) -> Self {
        match value {
            BinaryArithmeticOperator::Addition => Self::ADDITION,
            BinaryArithmeticOperator::Division => Self::DIVISION,
            BinaryArithmeticOperator::Multiplication => Self::MULTIPLICATION,
            BinaryArithmeticOperator::Subtraction => Self::SUBTRACTION,
        }
    }
}

impl Repr for PyBinaryArithmeticOperator {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self {
                Self::ADDITION => "ADDITION",
                Self::DIVISION => "DIVISION",
                Self::MULTIPLICATION => "MULTIPLICATION",
                Self::SUBTRACTION => "SUBTRACTION",
            }
        ))
    }
}
