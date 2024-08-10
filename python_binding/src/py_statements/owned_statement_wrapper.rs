use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python};

use az::parsing::Statement;

use crate::traits::Repr;
use crate::types::OwnedString;

use super::py_expression_statement::PyExpressionStatement;

#[derive(Clone, PartialEq)]
pub(crate) struct OwnedStatementWrapper(Statement<OwnedString>);

impl From<Statement<OwnedString>> for OwnedStatementWrapper {
    fn from(value: Statement<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<OwnedStatementWrapper> for Statement<OwnedString> {
    fn from(value: OwnedStatementWrapper) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for OwnedStatementWrapper {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.0 {
            Statement::Expression(value) => {
                PyExpressionStatement::from(value).into_py(py)
            }
        }
    }
}

impl<'source> FromPyObject<'source> for OwnedStatementWrapper {
    fn extract_bound(object: &Bound<'source, PyAny>) -> PyResult<Self> {
        object.extract::<PyExpressionStatement>().map(|value| {
            OwnedStatementWrapper(Statement::Expression(value.into()))
        })
    }
}

impl Repr for Statement<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            Statement::Expression(value) => value.repr(py),
        }
    }
}

impl Repr for OwnedStatementWrapper {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}
