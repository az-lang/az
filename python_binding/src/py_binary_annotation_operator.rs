use pyo3::{pyclass, pymethods};

use az::parsing::{Associativity, BinaryAnnotationOperator, Precedence};

use super::py_associativity::PyAssociativity;
use super::py_precedence::PyPrecedence;

#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "BinaryAnnotationOperator", frozen)]
pub(crate) struct PyBinaryAnnotationOperator;

impl PyBinaryAnnotationOperator {
    const OPERATOR: BinaryAnnotationOperator = BinaryAnnotationOperator;
}

#[pymethods]
impl PyBinaryAnnotationOperator {
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
