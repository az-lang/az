use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::Call;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "Call", extends = PyExpression, frozen
)]
pub(crate) struct PyCall(Call<OwnedStr>);

#[pymethods]
impl PyCall {
    #[getter]
    fn arguments(&self) -> Vec<OwnedExpressionWrapper> {
        self.0.arguments.iter().cloned().map(Into::into).collect()
    }

    #[getter]
    fn callable(&self) -> OwnedExpressionWrapper {
        self.0.callable.as_ref().clone().into()
    }

    #[getter]
    fn close_parenthesis_position(&self) -> PySubstringPosition {
        self.0.close_parenthesis_position.clone().into()
    }

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
    fn commas_fillers(&self) -> Vec<PyFillers> {
        self.0
            .commas_fillers
            .iter()
            .map(|comma_fillers| {
                comma_fillers.iter().cloned().map(Into::into).collect()
            })
            .collect()
    }

    #[getter]
    fn commas_positions(&self) -> Vec<PySubstringPosition> {
        self.0
            .commas_positions
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn open_parenthesis_position(&self) -> PySubstringPosition {
        self.0.open_parenthesis_position.clone().into()
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

    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (
        callable, arguments, /, *, open_parenthesis_position, commas_positions,
        close_parenthesis_position, open_parenthesis_fillers, commas_fillers,
        close_parenthesis_fillers,
    ))]
    fn new(
        callable: OwnedExpressionWrapper,
        arguments: Vec<OwnedExpressionWrapper>,
        open_parenthesis_position: PySubstringPosition,
        commas_positions: Vec<PySubstringPosition>,
        close_parenthesis_position: PySubstringPosition,
        open_parenthesis_fillers: PyFillers,
        commas_fillers: Vec<PyFillers>,
        close_parenthesis_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(Call {
            callable: Box::new(callable.into()),
            arguments: arguments.into_iter().map(Into::into).collect(),
            open_parenthesis_position: open_parenthesis_position.into(),
            commas_positions: commas_positions
                .into_iter()
                .map(Into::into)
                .collect(),
            close_parenthesis_position: close_parenthesis_position.into(),
            open_parenthesis_fillers: open_parenthesis_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            commas_fillers: commas_fillers
                .into_iter()
                .map(|comma_fillers| {
                    comma_fillers.into_iter().map(Into::into).collect()
                })
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

impl From<Call<OwnedStr>> for PyCall {
    fn from(value: Call<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyCall> for Call<OwnedStr> {
    fn from(value: PyCall) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyCall {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for Call<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, open_parenthesis_position={}, commas_positions={}, close_parenthesis_position={}, open_parenthesis_fillers={}, commas_fillers={}, close_parenthesis_fillers={})",
            PyCall::NAME,
            self.callable.repr(py)?,
            self.arguments.repr(py)?,
            self.open_parenthesis_position.repr(py)?,
            self.commas_positions.repr(py)?,
            self.close_parenthesis_position.repr(py)?,
            self.open_parenthesis_fillers.repr(py)?,
            self.commas_fillers.repr(py)?,
            self.close_parenthesis_fillers.repr(py)?
        ))
    }
}

impl Repr for PyCall {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyCall, PyExpression);
