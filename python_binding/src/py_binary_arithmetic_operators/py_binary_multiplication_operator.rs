use crate::py_associativity::PyAssociativity;
use crate::py_precedence::PyPrecedence;
use az::parsing::{Associativity, BinaryArithmeticOperator, Precedence};
use pyo3::{pyclass, pymethods};

#[derive(Clone, Eq, PartialEq)]
#[pyclass(
    module = "az.parsing",
    name = "BinaryMultiplicationOperator",
    frozen
)]
pub(crate) struct PyBinaryMultiplicationOperator;

impl PyBinaryMultiplicationOperator {
    const OPERATOR: BinaryArithmeticOperator =
        BinaryArithmeticOperator::Multiplication;
}

#[pymethods]
impl PyBinaryMultiplicationOperator {
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
