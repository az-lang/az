use pyo3::{pyclass, pymethods, Py, Python};

use az::parsing::{Associativity, MemberAccessOperator, Precedence};

use super::py_associativity::PyAssociativity;
use super::py_precedence::PyPrecedence;

#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "MemberAccessOperator", frozen)]
pub(crate) struct PyMemberAccessOperator;

impl PyMemberAccessOperator {
    const OPERATOR: MemberAccessOperator = MemberAccessOperator;
}

#[pymethods]
impl PyMemberAccessOperator {
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
