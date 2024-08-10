use pyo3::exceptions::PyTypeError;
use pyo3::types::PyAnyMethods;
use pyo3::{
    Bound, FromPyObject, IntoPy, PyAny, PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::BinaryComparisonOperator;

use crate::traits::Repr;

use super::py_binary_equal_to_operator::PyBinaryEqualToOperator;
use super::py_binary_greater_than_operator::PyBinaryGreaterThanOperator;
use super::py_binary_greater_than_or_equal_to_operator::PyBinaryGreaterThanOrEqualToOperator;
use super::py_binary_less_than_operator::PyBinaryLessThanOperator;
use super::py_binary_less_than_or_equal_to_operator::PyBinaryLessThanOrEqualToOperator;
use super::py_binary_not_equal_to_operator::PyBinaryNotEqualToOperator;

pub(crate) struct BinaryComparisonOperatorWrapper(BinaryComparisonOperator);

impl From<BinaryComparisonOperatorWrapper> for BinaryComparisonOperator {
    fn from(value: BinaryComparisonOperatorWrapper) -> Self {
        value.0
    }
}

impl From<BinaryComparisonOperator> for BinaryComparisonOperatorWrapper {
    fn from(value: BinaryComparisonOperator) -> Self {
        Self(value)
    }
}

impl IntoPy<PyObject> for BinaryComparisonOperatorWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            BinaryComparisonOperator::EqualTo => {
                PyBinaryEqualToOperator::type_object_bound(py).into()
            }
            BinaryComparisonOperator::GreaterThan => {
                PyBinaryGreaterThanOperator::type_object_bound(py).into()
            }
            BinaryComparisonOperator::GreaterThanOrEqualTo => {
                PyBinaryGreaterThanOrEqualToOperator::type_object_bound(py)
                    .into()
            }
            BinaryComparisonOperator::LessThan => {
                PyBinaryLessThanOperator::type_object_bound(py).into()
            }
            BinaryComparisonOperator::LessThanOrEqualTo => {
                PyBinaryLessThanOrEqualToOperator::type_object_bound(py).into()
            }
            BinaryComparisonOperator::NotEqualTo => {
                PyBinaryNotEqualToOperator::type_object_bound(py).into()
            }
        }
    }
}

impl<'source> FromPyObject<'source> for BinaryComparisonOperatorWrapper {
    fn extract_bound(object: &Bound<'source, PyAny>) -> PyResult<Self> {
        if object.is(&PyBinaryEqualToOperator::type_object_bound(object.py()))
        {
            Ok(BinaryComparisonOperatorWrapper(
                BinaryComparisonOperator::EqualTo,
            ))
        } else if object
            .is(&PyBinaryGreaterThanOperator::type_object_bound(object.py()))
        {
            Ok(BinaryComparisonOperatorWrapper(
                BinaryComparisonOperator::GreaterThan,
            ))
        } else if object.is(
            &PyBinaryGreaterThanOrEqualToOperator::type_object_bound(
                object.py(),
            ),
        ) {
            Ok(BinaryComparisonOperatorWrapper(
                BinaryComparisonOperator::GreaterThanOrEqualTo,
            ))
        } else if object
            .is(&PyBinaryLessThanOperator::type_object_bound(object.py()))
        {
            Ok(BinaryComparisonOperatorWrapper(
                BinaryComparisonOperator::LessThan,
            ))
        } else if object.is(
            &PyBinaryLessThanOrEqualToOperator::type_object_bound(object.py()),
        ) {
            Ok(BinaryComparisonOperatorWrapper(
                BinaryComparisonOperator::LessThanOrEqualTo,
            ))
        } else if object
            .is(&PyBinaryNotEqualToOperator::type_object_bound(object.py()))
        {
            Ok(BinaryComparisonOperatorWrapper(
                BinaryComparisonOperator::NotEqualTo,
            ))
        } else {
            Err(PyTypeError::new_err(format!(
                "{} is not a binary comparison operator.",
                object.repr()?
            )))
        }
    }
}

impl Repr for BinaryComparisonOperator {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            BinaryComparisonOperator::EqualTo => {
                PyBinaryEqualToOperator::type_object_bound(py).repr(py)
            }
            BinaryComparisonOperator::GreaterThan => {
                PyBinaryGreaterThanOperator::type_object_bound(py).repr(py)
            }
            BinaryComparisonOperator::GreaterThanOrEqualTo => {
                PyBinaryGreaterThanOrEqualToOperator::type_object_bound(py)
                    .repr(py)
            }
            BinaryComparisonOperator::LessThan => {
                PyBinaryLessThanOperator::type_object_bound(py).repr(py)
            }
            BinaryComparisonOperator::LessThanOrEqualTo => {
                PyBinaryLessThanOrEqualToOperator::type_object_bound(py)
                    .repr(py)
            }
            BinaryComparisonOperator::NotEqualTo => {
                PyBinaryNotEqualToOperator::type_object_bound(py).repr(py)
            }
        }
    }
}
