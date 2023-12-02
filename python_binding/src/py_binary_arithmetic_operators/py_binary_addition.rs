use crate::py_associativity::PyAssociativity;
use crate::py_precedence::PyPrecedence;
use az::parsing::{Associativity, BinaryArithmeticOperator, Precedence};
use pyo3::{pyclass, pymethods};

#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "BinaryAdditionOperator", frozen)]
pub(crate) struct PyBinaryAdditionOperator;

impl PyBinaryAdditionOperator {
    const OPERATOR: BinaryArithmeticOperator =
        BinaryArithmeticOperator::Addition;
}

#[pymethods]
impl PyBinaryAdditionOperator {
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
