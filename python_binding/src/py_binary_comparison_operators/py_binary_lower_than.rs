use pyo3::{pyclass, pymethods};

use az::parsing::{Associativity, BinaryComparisonOperator, Precedence};

use crate::py_associativity::PyAssociativity;
use crate::py_precedence::PyPrecedence;

#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "BinaryLowerThanOperator", frozen)]
pub(crate) struct PyBinaryLowerThanOperator;

impl PyBinaryLowerThanOperator {
    const OPERATOR: BinaryComparisonOperator =
        BinaryComparisonOperator::LowerThan;
}

#[pymethods]
impl PyBinaryLowerThanOperator {
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
