use pyo3::exceptions::PyTypeError;
use pyo3::{
    pyclass, pymethods, FromPyObject, IntoPy, PyAny, PyObject, PyResult,
    PyTypeInfo, Python,
};

use az::parsing::{Precedence, UnaryArithmeticOperator};

use super::py_precedence::PyPrecedence;
use super::traits::Repr;

#[derive(Clone, Eq, PartialEq)]
pub(super) struct UnaryArithmeticOperatorWrapper(UnaryArithmeticOperator);

#[derive(Clone, Eq, PartialEq)]
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
    fn extract(object: &'source PyAny) -> PyResult<Self> {
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
