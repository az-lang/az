use az::parsing::ComparisonOperator;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "ComparisonOperator", frozen)]
pub(crate) enum PyComparisonOperator {
    EQUAL_TO,
    GREATER_THAN,
    GREATER_THAN_OR_EQUAL_TO,
    LOWER_THAN,
    LOWER_THAN_OR_EQUAL_TO,
    NOT_EQUAL_TO,
}

#[pymethods]
impl PyComparisonOperator {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<ComparisonOperator> for PyComparisonOperator {
    fn from(value: ComparisonOperator) -> Self {
        match value {
            ComparisonOperator::EqualTo => Self::EQUAL_TO,
            ComparisonOperator::GreaterThan => Self::GREATER_THAN,
            ComparisonOperator::GreaterThanOrEqualTo => {
                Self::GREATER_THAN_OR_EQUAL_TO
            }
            ComparisonOperator::LowerThan => Self::LOWER_THAN,
            ComparisonOperator::LowerThanOrEqualTo => {
                Self::LOWER_THAN_OR_EQUAL_TO
            }
            ComparisonOperator::NotEqualTo => Self::NOT_EQUAL_TO,
        }
    }
}

impl Repr for PyComparisonOperator {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self {
                Self::EQUAL_TO => "EQUAL_TO",
                Self::GREATER_THAN => "GREATER_THAN",
                Self::GREATER_THAN_OR_EQUAL_TO => "GREATER_THAN_OR_EQUAL_TO",
                Self::LOWER_THAN => "LOWER_THAN",
                Self::LOWER_THAN_OR_EQUAL_TO => "LOWER_THAN_OR_EQUAL_TO",
                Self::NOT_EQUAL_TO => "NOT_EQUAL_TO",
            }
        ))
    }
}
