use az::parsing::Block;
use pyo3::types::{PyDict, PyDictMethods, PyTuple};
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::{PyFiller, PyFillers};
use crate::py_statements::OwnedStatementWrapper;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::owned_expression_wrapper::OwnedExpressionWrapper;
use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "Block", extends = PyExpression, frozen
)]
pub(crate) struct PyBlock(Block<OwnedString>);

#[pymethods]
impl PyBlock {
    #[new]
    #[pyo3(
    signature = (
    statements, expression, /, *, open_brace_position,
    close_brace_position, open_brace_fillers, close_brace_fillers
    )
    )]
    fn new(
        statements: Vec<OwnedStatementWrapper>,
        expression: Option<OwnedExpressionWrapper>,
        open_brace_position: PySubstringPosition,
        close_brace_position: PySubstringPosition,
        open_brace_fillers: PyFillers,
        close_brace_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(Block {
            statements: statements.into_iter().map(Into::into).collect(),
            expression: expression
                .map(|expression| Box::new(expression.into())),
            open_brace_position: open_brace_position.into(),
            close_brace_position: close_brace_position.into(),
            open_brace_fillers: open_brace_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
            close_brace_fillers: close_brace_fillers
                .into_iter()
                .map(Into::into)
                .collect(),
        }))
    }

    #[getter]
    fn close_brace_position(&self) -> PySubstringPosition {
        self.0.close_brace_position.clone().into()
    }

    #[getter]
    fn close_brace_fillers(&self) -> PyFillers {
        self.0
            .close_brace_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn expression(&self) -> Option<OwnedExpressionWrapper> {
        self.0
            .expression
            .as_ref()
            .map(|expression| expression.as_ref().clone().into())
    }

    #[getter]
    fn open_brace_position(&self) -> PySubstringPosition {
        self.0.open_brace_position.clone().into()
    }

    #[getter]
    fn open_brace_fillers(&self) -> PyFillers {
        self.0
            .open_brace_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn statements(&self) -> Vec<OwnedStatementWrapper> {
        self.0.statements.iter().cloned().map(Into::into).collect()
    }

    fn validate_contents(&self, py: Python<'_>) -> PyResult<()> {
        validate_contents(&self.0, py)
    }

    fn validate_positions(&self, py: Python<'_>) -> PyResult<()> {
        validate_positions(&self.0, py)
    }

    fn __getnewargs_ex__<'py>(
        &'py self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyTuple>, Bound<'py, PyDict>)> {
        let kwargs = PyDict::new_bound(py);
        kwargs.set_item(
            "open_brace_position",
            PySubstringPosition::from(self.0.open_brace_position.clone())
                .into_py(py),
        )?;
        kwargs.set_item(
            "close_brace_position",
            PySubstringPosition::from(self.0.close_brace_position.clone())
                .into_py(py),
        )?;
        kwargs.set_item(
            "open_brace_fillers",
            self.0
                .open_brace_fillers
                .iter()
                .cloned()
                .map(|value| PyFiller::from(value).into_py(py))
                .collect::<Vec<_>>(),
        )?;
        kwargs.set_item(
            "close_brace_fillers",
            self.0
                .close_brace_fillers
                .iter()
                .cloned()
                .map(|value| PyFiller::from(value).into_py(py))
                .collect::<Vec<_>>(),
        )?;
        Ok((
            PyTuple::new_bound(
                py,
                &[
                    self.0
                        .statements
                        .iter()
                        .cloned()
                        .map(|value| {
                            OwnedStatementWrapper::from(value).into_py(py)
                        })
                        .collect::<Vec<_>>()
                        .into_py(py),
                    self.0
                        .expression
                        .as_ref()
                        .map(|value| {
                            OwnedExpressionWrapper::from(
                                value.as_ref().clone(),
                            )
                            .into_py(py)
                        })
                        .into_py(py),
                ],
            ),
            kwargs,
        ))
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

impl From<Block<OwnedString>> for PyBlock {
    fn from(value: Block<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyBlock> for Block<OwnedString> {
    fn from(value: PyBlock) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyBlock {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for Block<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, open_brace_position={}, close_brace_position={}, open_brace_fillers={}, close_brace_fillers={})",
            PyBlock::NAME,
            self.statements.repr(py)?,
            self.expression.repr(py)?,
            self.open_brace_position.repr(py)?,
            self.close_brace_position.repr(py)?,
            self.open_brace_fillers.repr(py)?,
            self.close_brace_fillers.repr(py)?
        ))
    }
}

impl Repr for PyBlock {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyBlock, PyExpression);
