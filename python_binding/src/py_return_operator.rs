use pyo3::{pyclass, pymethods};

use az::parsing::{Precedence, ReturnOperator};

use crate::py_precedence::PyPrecedence;

#[pyclass(module = "az.parsing", name = "ReturnOperator", frozen)]
pub(crate) struct PyReturnOperator;

impl PyReturnOperator {
    const OPERATOR: ReturnOperator = ReturnOperator;
}

#[pymethods]
impl PyReturnOperator {
    #[allow(non_snake_case)]
    #[classattr]
    fn PRECEDENCE() -> PyPrecedence {
        Precedence::from(Self::OPERATOR).into()
    }
}
