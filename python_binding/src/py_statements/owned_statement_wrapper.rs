use pyo3::{FromPyObject, IntoPy, PyAny, PyObject, PyResult, Python};

use az::parsing::Statement;

use crate::traits::Repr;
use crate::types::OwnedStr;

use super::py_expression_statement::PyExpressionStatement;

#[derive(Clone, PartialEq)]
pub(crate) struct OwnedStatementWrapper(Statement<OwnedStr>);

impl From<Statement<OwnedStr>> for OwnedStatementWrapper {
    fn from(value: Statement<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<OwnedStatementWrapper> for Statement<OwnedStr> {
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
    fn extract(object: &'source PyAny) -> PyResult<Self> {
        object.extract::<PyExpressionStatement>().map(|value| {
            OwnedStatementWrapper(Statement::Expression(value.into()))
        })
    }
}

impl Repr for Statement<OwnedStr> {
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
