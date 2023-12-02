use pyo3::{pyclass, pymethods};

use az::parsing::{Associativity, CallOperator, Precedence};

use super::py_associativity::PyAssociativity;
use super::py_precedence::PyPrecedence;

#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "CallOperator", frozen)]
pub(crate) struct PyCallOperator;

impl PyCallOperator {
    const OPERATOR: CallOperator = CallOperator;
}

#[pymethods]
impl PyCallOperator {
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
