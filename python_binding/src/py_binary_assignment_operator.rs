use pyo3::{pyclass, pymethods};

use az::parsing::{Associativity, BinaryAssignmentOperator, Precedence};

use super::py_associativity::PyAssociativity;
use super::py_precedence::PyPrecedence;

#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "BinaryAssignmentOperator", frozen)]
pub(crate) struct PyBinaryAssignmentOperator;

impl PyBinaryAssignmentOperator {
    const OPERATOR: BinaryAssignmentOperator = BinaryAssignmentOperator;
}

#[pymethods]
impl PyBinaryAssignmentOperator {
    #[allow(non_snake_case)]
    #[classattr]
    fn ASSOCIATIVITY() -> PyAssociativity {
        Associativity::from(Self::OPERATOR).into()
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PRECEDENCE() -> PyPrecedence {
        Precedence::from(Self::OPERATOR).into()
    }
}
