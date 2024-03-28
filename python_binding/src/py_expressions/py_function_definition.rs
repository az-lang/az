use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::FunctionDefinition;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_block::PyBlock;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "FunctionDefinition", extends = PyExpression,
    frozen
)]
pub(crate) struct PyFunctionDefinition(FunctionDefinition<OwnedStr>);

#[pymethods]
impl PyFunctionDefinition {
    #[getter]
    fn arrow_fillers(&self) -> PyFillers {
        self.0
            .arrow_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn arrow_position(&self) -> PySubstringPosition {
        self.0.arrow_position.clone().into()
    }

    #[getter]
    fn body(&self) -> PyBlock {
        self.0.body.clone().into()
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
    fn close_parenthesis_position(&self) -> PySubstringPosition {
        self.0.close_parenthesis_position.clone().into()
    }

    #[getter]
    fn commas_fillers(&self) -> Vec<PyFillers> {
        self.0
            .commas_fillers
            .iter()
            .map(|fillers| fillers.iter().cloned().map(Into::into).collect())
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

    #[getter]
    fn opener_fillers(&self) -> PyFillers {
        self.0
            .opener_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn opener_position(&self) -> PySubstringPosition {
        self.0.opener_position.clone().into()
    }

    #[getter]
    fn parameters(&self) -> Vec<OwnedExpressionWrapper> {
        self.0.parameters.iter().cloned().map(Into::into).collect()
    }

    #[getter]
    fn return_type(&self) -> OwnedExpressionWrapper {
        self.0.return_type.as_ref().clone().into()
    }

    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (
        parameters, return_type, body, /, *, opener_position,
        open_parenthesis_position, commas_positions,
        close_parenthesis_position, arrow_position, opener_fillers,
        open_parenthesis_fillers, commas_fillers, close_parenthesis_fillers,
        arrow_fillers,
    ))]
    fn new(
        parameters: Vec<OwnedExpressionWrapper>,
        return_type: OwnedExpressionWrapper,
        body: PyBlock,
        opener_position: PySubstringPosition,
        open_parenthesis_position: PySubstringPosition,
        commas_positions: Vec<PySubstringPosition>,
        close_parenthesis_position: PySubstringPosition,
        arrow_position: PySubstringPosition,
        opener_fillers: PyFillers,
        open_parenthesis_fillers: PyFillers,
        commas_fillers: Vec<PyFillers>,
        close_parenthesis_fillers: PyFillers,
        arrow_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(FunctionDefinition {
            parameters: parameters.into_iter().map(Into::into).collect(),
            return_type: Box::new(return_type.into()),
            body: body.into(),
            opener_position: opener_position.into(),
            open_parenthesis_position: open_parenthesis_position.into(),
            commas_positions: commas_positions
                .into_iter()
                .map(Into::into)
                .collect(),
            close_parenthesis_position: close_parenthesis_position.into(),
            arrow_position: arrow_position.into(),
            opener_fillers: opener_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            open_parenthesis_fillers: open_parenthesis_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            commas_fillers: commas_fillers
                .into_iter()
                .map(|fillers| fillers.into_iter().map(Into::into).collect())
                .collect(),
            close_parenthesis_fillers: close_parenthesis_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            arrow_fillers: arrow_fillers.into_iter().map(Into::into).collect(),
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

impl From<FunctionDefinition<OwnedStr>> for PyFunctionDefinition {
    fn from(value: FunctionDefinition<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyFunctionDefinition> for FunctionDefinition<OwnedStr> {
    fn from(value: PyFunctionDefinition) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyFunctionDefinition {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for FunctionDefinition<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, opener_position={}, open_parenthesis_position={}, commas_positions={}, close_parenthesis_position={}, arrow_position={}, opener_fillers={}, open_parenthesis_fillers={}, commas_fillers={}, close_parenthesis_fillers={}, arrow_fillers={})",
            PyFunctionDefinition::NAME,
            self.parameters.repr(py)?,
            self.return_type.repr(py)?,
            self.body.repr(py)?,
            self.opener_position.repr(py)?,
            self.open_parenthesis_position.repr(py)?,
            self.commas_positions.repr(py)?,
            self.close_parenthesis_position.repr(py)?,
            self.arrow_position.repr(py)?,
            self.opener_fillers.repr(py)?,
            self.open_parenthesis_fillers.repr(py)?,
            self.commas_fillers.repr(py)?,
            self.close_parenthesis_fillers.repr(py)?,
            self.arrow_fillers.repr(py)?
        ))
    }
}

impl Repr for PyFunctionDefinition {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyFunctionDefinition,
    PyExpression
);
