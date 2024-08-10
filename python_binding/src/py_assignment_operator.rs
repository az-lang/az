use pyo3::{pyclass, pymethods, Py, Python};

use az::parsing::{AssignmentOperator, Associativity, Precedence};

use super::py_associativity::PyAssociativity;
use super::py_precedence::PyPrecedence;

#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "AssignmentOperator", frozen)]
pub(crate) struct PyAssignmentOperator;

impl PyAssignmentOperator {
    const OPERATOR: AssignmentOperator = AssignmentOperator;
}

#[pymethods]
impl PyAssignmentOperator {
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
