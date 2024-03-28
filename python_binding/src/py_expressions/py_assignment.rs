use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::Assignment;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "Assignment", extends = PyExpression, frozen
)]
pub(crate) struct PyAssignment(Assignment<OwnedStr>);

#[pymethods]
impl PyAssignment {
    #[getter]
    fn operator_fillers(&self) -> PyFillers {
        self.0
            .operator_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn operator_position(&self) -> PySubstringPosition {
        self.0.operator_position.clone().into()
    }

    #[getter]
    fn target(&self) -> OwnedExpressionWrapper {
        self.0.target.as_ref().clone().into()
    }

    #[getter]
    fn value(&self) -> OwnedExpressionWrapper {
        self.0.value.as_ref().clone().into()
    }

    #[new]
    #[pyo3(signature = (target, value, /, *, operator_position, operator_fillers))]
    fn new(
        target: OwnedExpressionWrapper,
        value: OwnedExpressionWrapper,
        operator_position: PySubstringPosition,
        operator_fillers: PyFillers,
    ) -> PyClassInitializer<PyAssignment> {
        PyExpression::new().add_subclass(Self(Assignment {
            target: Box::new(target.into()),
            value: Box::new(value.into()),
            operator_position: operator_position.into(),
            operator_fillers: operator_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
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

impl From<Assignment<OwnedStr>> for PyAssignment {
    fn from(value: Assignment<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyAssignment> for Assignment<OwnedStr> {
    fn from(value: PyAssignment) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyAssignment {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for Assignment<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, operator_position={}, operator_fillers={})",
            PyAssignment::NAME,
            self.target.repr(py)?,
            self.value.repr(py)?,
            self.operator_position.repr(py)?,
            self.operator_fillers.repr(py)?
        ))
    }
}

impl Repr for PyAssignment {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyAssignment, PyExpression);
