use pyo3::{pyclass, pymethods};

use az::parsing::{Associativity, BinaryComparisonOperator, Precedence};

use crate::py_associativity::PyAssociativity;
use crate::py_precedence::PyPrecedence;

#[derive(Clone, Eq, PartialEq)]
#[pyclass(
    module = "az.parsing",
    name = "BinaryLowerThanOrEqualToOperator",
    frozen
)]
pub(crate) struct PyBinaryLowerThanOrEqualToOperator;

impl PyBinaryLowerThanOrEqualToOperator {
    const OPERATOR: BinaryComparisonOperator =
        BinaryComparisonOperator::LowerThanOrEqualTo;
}

#[pymethods]
impl PyBinaryLowerThanOrEqualToOperator {
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
