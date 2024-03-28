use pyo3::exceptions::PyTypeError;
use pyo3::{
    FromPyObject, IntoPy, PyAny, PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::BinaryArithmeticOperator;

use crate::traits::Repr;

use super::py_binary_addition::PyBinaryAdditionOperator;
use super::py_binary_division_operator::PyBinaryDivisionOperator;
use super::py_binary_multiplication_operator::PyBinaryMultiplicationOperator;
use super::py_binary_subtraction_operator::PyBinarySubtractionOperator;

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct BinaryArithmeticOperatorWrapper(BinaryArithmeticOperator);

impl From<BinaryArithmeticOperatorWrapper> for BinaryArithmeticOperator {
    fn from(value: BinaryArithmeticOperatorWrapper) -> Self {
        value.0
    }
}

impl From<BinaryArithmeticOperator> for BinaryArithmeticOperatorWrapper {
    fn from(value: BinaryArithmeticOperator) -> Self {
        Self(value)
    }
}

impl IntoPy<PyObject> for BinaryArithmeticOperatorWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            BinaryArithmeticOperator::Addition => {
                PyBinaryAdditionOperator::type_object_bound(py).into()
            }
            BinaryArithmeticOperator::Division => {
                PyBinaryDivisionOperator::type_object_bound(py).into()
            }
            BinaryArithmeticOperator::Multiplication => {
                PyBinaryMultiplicationOperator::type_object_bound(py).into()
            }
            BinaryArithmeticOperator::Subtraction => {
                PyBinarySubtractionOperator::type_object_bound(py).into()
            }
        }
    }
}

impl<'source> FromPyObject<'source> for BinaryArithmeticOperatorWrapper {
    fn extract(object: &'source PyAny) -> PyResult<Self> {
        if object.is(&PyBinaryAdditionOperator::type_object_bound(object.py()))
        {
            Ok(BinaryArithmeticOperatorWrapper(
                BinaryArithmeticOperator::Addition,
            ))
        } else if object
            .is(&PyBinaryDivisionOperator::type_object_bound(object.py()))
        {
            Ok(BinaryArithmeticOperatorWrapper(
                BinaryArithmeticOperator::Division,
            ))
        } else if object.is(
            &PyBinaryMultiplicationOperator::type_object_bound(object.py()),
        ) {
            Ok(BinaryArithmeticOperatorWrapper(
                BinaryArithmeticOperator::Multiplication,
            ))
        } else if object
            .is(&PyBinarySubtractionOperator::type_object_bound(object.py()))
        {
            Ok(BinaryArithmeticOperatorWrapper(
                BinaryArithmeticOperator::Subtraction,
            ))
        } else {
            Err(PyTypeError::new_err(format!(
                "{} is not a binary arithmetic operator.",
                object.repr()?
            )))
        }
    }
}

impl Repr for BinaryArithmeticOperator {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            BinaryArithmeticOperator::Addition => {
                PyBinaryAdditionOperator::type_object_bound(py).repr(py)
            }
            BinaryArithmeticOperator::Division => {
                PyBinaryDivisionOperator::type_object_bound(py).repr(py)
            }
            BinaryArithmeticOperator::Multiplication => {
                PyBinaryMultiplicationOperator::type_object_bound(py).repr(py)
            }
            BinaryArithmeticOperator::Subtraction => {
                PyBinarySubtractionOperator::type_object_bound(py).repr(py)
            }
        }
    }
}
