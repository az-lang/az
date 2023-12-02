use pyo3::exceptions::PyTypeError;
use pyo3::{
    FromPyObject, IntoPy, PyAny, PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::BinaryComparisonOperator;

use crate::traits::Repr;

use super::py_binary_equal_to::PyBinaryEqualToOperator;
use super::py_binary_greater_than::PyBinaryGreaterThanOperator;
use super::py_binary_greater_than_or_equal_to::PyBinaryGreaterThanOrEqualToOperator;
use super::py_binary_lower_than::PyBinaryLowerThanOperator;
use super::py_binary_lower_than_or_equal_to::PyBinaryLowerThanOrEqualToOperator;
use super::py_binary_not_equal_to::PyBinaryNotEqualToOperator;

#[derive(Clone, Eq, PartialEq)]
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
            BinaryComparisonOperator::LowerThan => {
                PyBinaryLowerThanOperator::type_object_bound(py).into()
            }
            BinaryComparisonOperator::LowerThanOrEqualTo => {
                PyBinaryLowerThanOrEqualToOperator::type_object_bound(py)
                    .into()
            }
            BinaryComparisonOperator::NotEqualTo => {
                PyBinaryNotEqualToOperator::type_object_bound(py).into()
            }
        }
    }
}

impl<'source> FromPyObject<'source> for BinaryComparisonOperatorWrapper {
    fn extract(object: &'source PyAny) -> PyResult<Self> {
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
            .is(&PyBinaryLowerThanOperator::type_object_bound(object.py()))
        {
            Ok(BinaryComparisonOperatorWrapper(
                BinaryComparisonOperator::LowerThan,
            ))
        } else if object.is(
            &PyBinaryLowerThanOrEqualToOperator::type_object_bound(
                object.py(),
            ),
        ) {
            Ok(BinaryComparisonOperatorWrapper(
                BinaryComparisonOperator::LowerThanOrEqualTo,
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
            BinaryComparisonOperator::LowerThan => {
                PyBinaryLowerThanOperator::type_object_bound(py).repr(py)
            }
            BinaryComparisonOperator::LowerThanOrEqualTo => {
                PyBinaryLowerThanOrEqualToOperator::type_object_bound(py)
                    .repr(py)
            }
            BinaryComparisonOperator::NotEqualTo => {
                PyBinaryNotEqualToOperator::type_object_bound(py).repr(py)
            }
        }
    }
}
