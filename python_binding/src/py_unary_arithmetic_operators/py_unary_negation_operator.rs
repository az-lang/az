use pyo3::{pyclass, pymethods};

use az::parsing::{Precedence, UnaryArithmeticOperator};

use crate::py_precedence::PyPrecedence;

#[pyclass(module = "az.parsing", name = "UnaryNegationOperator", frozen)]
pub(crate) struct PyUnaryNegationOperator;

impl PyUnaryNegationOperator {
    const OPERATOR: UnaryArithmeticOperator =
        UnaryArithmeticOperator::Negation;
}

#[pymethods]
impl PyUnaryNegationOperator {
    #[allow(non_snake_case)]
    #[classattr]
    fn PRECEDENCE() -> PyPrecedence {
        Precedence::from(Self::OPERATOR).into()
    }
}
