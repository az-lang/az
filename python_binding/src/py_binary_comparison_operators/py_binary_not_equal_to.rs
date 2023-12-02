use pyo3::{pyclass, pymethods};

use az::parsing::{Associativity, BinaryComparisonOperator, Precedence};

use crate::py_associativity::PyAssociativity;
use crate::py_precedence::PyPrecedence;

#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "BinaryNotEqualToOperator", frozen)]
pub(crate) struct PyBinaryNotEqualToOperator;

impl PyBinaryNotEqualToOperator {
    const OPERATOR: BinaryComparisonOperator =
        BinaryComparisonOperator::NotEqualTo;
}

#[pymethods]
impl PyBinaryNotEqualToOperator {
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
