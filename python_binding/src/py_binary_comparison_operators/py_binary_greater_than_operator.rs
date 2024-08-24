use pyo3::{pyclass, pymethods, Py, Python};

use az::parsing::{Associativity, BinaryComparisonOperator, Precedence};

use crate::py_associativity::PyAssociativity;
use crate::py_precedence::PyPrecedence;

#[pyclass(module = "az.parsing", name = "BinaryGreaterThanOperator", frozen)]
pub(crate) struct PyBinaryGreaterThanOperator;

impl PyBinaryGreaterThanOperator {
    const OPERATOR: BinaryComparisonOperator =
        BinaryComparisonOperator::GreaterThan;
}

#[pymethods]
impl PyBinaryGreaterThanOperator {
    #[allow(non_snake_case)]
    #[classattr]
    fn ASSOCIATIVITY(py: Python<'_>) -> Py<PyAssociativity> {
        PyAssociativity::from_rust(Associativity::from(Self::OPERATOR), py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PRECEDENCE() -> PyPrecedence {
        Precedence::from(Self::OPERATOR).into()
    }
}
