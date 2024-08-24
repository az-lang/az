use pyo3::exceptions::PyTypeError;
use pyo3::types::PyAnyMethods;
use pyo3::{
    Bound, FromPyObject, IntoPy, PyAny, PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::UnaryArithmeticOperator;

use crate::traits::Repr;

use super::py_unary_negation_operator::PyUnaryNegationOperator;

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct UnaryArithmeticOperatorWrapper(UnaryArithmeticOperator);

impl From<UnaryArithmeticOperator> for UnaryArithmeticOperatorWrapper {
    fn from(value: UnaryArithmeticOperator) -> Self {
        Self(value)
    }
}

impl From<UnaryArithmeticOperatorWrapper> for UnaryArithmeticOperator {
    fn from(value: UnaryArithmeticOperatorWrapper) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for UnaryArithmeticOperatorWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            UnaryArithmeticOperator::Negation => {
                PyUnaryNegationOperator::type_object_bound(py).into()
            }
        }
    }
}

impl<'source> FromPyObject<'source> for UnaryArithmeticOperatorWrapper {
    fn extract_bound(object: &Bound<'source, PyAny>) -> PyResult<Self> {
        if object.is(&PyUnaryNegationOperator::type_object_bound(object.py()))
        {
            Ok(UnaryArithmeticOperatorWrapper(
                UnaryArithmeticOperator::Negation,
            ))
        } else {
            Err(PyTypeError::new_err(format!(
                "{} is not unary arithmetic operator.",
                object.repr()?
            )))
        }
    }
}

impl Repr for UnaryArithmeticOperator {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            UnaryArithmeticOperator::Negation => {
                PyUnaryNegationOperator::type_object_bound(py).repr(py)
            }
        }
    }
}
