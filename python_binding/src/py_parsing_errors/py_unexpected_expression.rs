use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::parsing::UnexpectedExpression;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_expressions::OwnedExpressionWrapper;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::py_parsing_error::PyParsingError;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "UnexpectedExpression",
    extends = PyParsingError, frozen
)]
pub(crate) struct PyUnexpectedExpression(UnexpectedExpression<OwnedStr>);

#[pymethods]
impl PyUnexpectedExpression {
    #[getter]
    fn expression(&self) -> OwnedExpressionWrapper {
        self.0.expression.clone().into()
    }

    #[new]
    #[pyo3(signature = (expression, /))]
    fn new(expression: OwnedExpressionWrapper) -> PyClassInitializer<Self> {
        PyParsingError::new().add_subclass(Self(UnexpectedExpression {
            expression: expression.into(),
        }))
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: pyclass::CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        self.rich_cmp(other, op, py)
    }
}

impl From<PyUnexpectedExpression> for PyErr {
    fn from(value: PyUnexpectedExpression) -> Self {
        Self::new::<PyUnexpectedExpression, _>((value.expression(),))
    }
}

impl From<UnexpectedExpression<OwnedStr>> for PyUnexpectedExpression {
    fn from(value: UnexpectedExpression<OwnedStr>) -> Self {
        Self(value)
    }
}

impl Repr for UnexpectedExpression<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            PyUnexpectedExpression::NAME,
            self.expression.repr(py)?
        ))
    }
}

impl Repr for PyUnexpectedExpression {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyUnexpectedExpression,
    PyParsingError
);
