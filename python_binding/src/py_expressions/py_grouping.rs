use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::Grouping;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "Grouping", extends = PyExpression, frozen
)]
pub(crate) struct PyGrouping(Grouping<OwnedStr>);

#[pymethods]
impl PyGrouping {
    #[getter]
    fn close_parenthesis_fillers(&self) -> PyFillers {
        self.0
            .close_parenthesis_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn close_parenthesis_position(&self) -> PySubstringPosition {
        self.0.close_parenthesis_position.clone().into()
    }

    #[getter]
    fn expression(&self) -> OwnedExpressionWrapper {
        self.0.expression.as_ref().clone().into()
    }

    #[getter]
    fn open_parenthesis_fillers(&self) -> PyFillers {
        self.0
            .open_parenthesis_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn open_parenthesis_position(&self) -> PySubstringPosition {
        self.0.open_parenthesis_position.clone().into()
    }

    #[new]
    #[pyo3(signature = (
        expression, /, *, open_parenthesis_position,
        close_parenthesis_position, open_parenthesis_fillers,
        close_parenthesis_fillers,
    ))]
    fn new(
        expression: OwnedExpressionWrapper,
        open_parenthesis_position: PySubstringPosition,
        close_parenthesis_position: PySubstringPosition,
        open_parenthesis_fillers: PyFillers,
        close_parenthesis_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(Grouping {
            expression: Box::new(expression.into()),
            open_parenthesis_position: open_parenthesis_position.into(),
            close_parenthesis_position: close_parenthesis_position.into(),
            open_parenthesis_fillers: open_parenthesis_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            close_parenthesis_fillers: close_parenthesis_fillers
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

impl From<Grouping<OwnedStr>> for PyGrouping {
    fn from(value: Grouping<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyGrouping> for Grouping<OwnedStr> {
    fn from(value: PyGrouping) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyGrouping {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for Grouping<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, open_parenthesis_position={}, close_parenthesis_position={}, open_parenthesis_fillers={}, close_parenthesis_fillers={})",
            PyGrouping::NAME,
            self.expression.repr(py)?,
            self.open_parenthesis_position.repr(py)?,
            self.close_parenthesis_position.repr(py)?,
            self.open_parenthesis_fillers.repr(py)?,
            self.close_parenthesis_fillers.repr(py)?,
        ))
    }
}

impl Repr for PyGrouping {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyGrouping, PyExpression);
