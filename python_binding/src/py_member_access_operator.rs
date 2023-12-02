use pyo3::{pyclass, pymethods};

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
    fn ASSOCIATIVITY() -> PyAssociativity {
        Associativity::from(Self::OPERATOR).into()
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn PRECEDENCE() -> PyPrecedence {
        Precedence::from(Self::OPERATOR).into()
    }
}
