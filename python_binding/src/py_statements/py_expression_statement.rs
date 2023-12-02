use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::ExpressionStatement;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_expressions::OwnedExpressionWrapper;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::py_statement::PyStatement;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "ExpressionStatement", extends = PyStatement,
    frozen
)]
pub(crate) struct PyExpressionStatement(ExpressionStatement<OwnedStr>);

#[pymethods]
impl PyExpressionStatement {
    #[getter]
    fn expression(&self) -> OwnedExpressionWrapper {
        self.0.expression.clone().into()
    }

    #[getter]
    fn semicolon_fillers(&self) -> PyFillers {
        self.0
            .semicolon_fillers
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }

    #[getter]
    fn semicolon_position(&self) -> PySubstringPosition {
        self.0.semicolon_position.clone().into()
    }

    #[new]
    #[pyo3(signature = (expression, /, *, semicolon_position, semicolon_fillers))]
    fn new(
        expression: OwnedExpressionWrapper,
        semicolon_position: PySubstringPosition,
        semicolon_fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyStatement::new().add_subclass(Self(ExpressionStatement {
            expression: expression.into(),
            semicolon_position: semicolon_position.into(),
            semicolon_fillers: semicolon_fillers
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

impl From<PyExpressionStatement> for ExpressionStatement<OwnedStr> {
    fn from(value: PyExpressionStatement) -> Self {
        value.0
    }
}

impl From<ExpressionStatement<OwnedStr>> for PyExpressionStatement {
    fn from(value: ExpressionStatement<OwnedStr>) -> Self {
        Self(value)
    }
}

impl IntoPy<PyObject> for PyExpressionStatement {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyStatement::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for ExpressionStatement<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, semicolon_position={}, semicolon_fillers={})",
            PyExpressionStatement::NAME,
            self.expression.repr(py)?,
            self.semicolon_position.repr(py)?,
            self.semicolon_fillers.repr(py)?
        ))
    }
}

impl Repr for PyExpressionStatement {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyExpressionStatement,
    PyStatement
);
