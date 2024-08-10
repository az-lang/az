use pyo3::{pyclass, pymethods, Py, Python};

use az::parsing::{Associativity, BinaryComparisonOperator, Precedence};

use crate::py_associativity::PyAssociativity;
use crate::py_precedence::PyPrecedence;

#[pyclass(
    module = "az.parsing",
    name = "BinaryLessThanOrEqualToOperator",
    frozen
)]
pub(crate) struct PyBinaryLessThanOrEqualToOperator;

impl PyBinaryLessThanOrEqualToOperator {
    const OPERATOR: BinaryComparisonOperator =
        BinaryComparisonOperator::LessThanOrEqualTo;
}

#[pymethods]
impl PyBinaryLessThanOrEqualToOperator {
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
