use pyo3::{
    pyclass, pymethods, IntoPy, Py, PyClassInitializer, PyObject, PyResult,
    PyTypeInfo, Python,
};

use az::parsing::Statement;

use super::py_expression::OwnedExpression;
use super::traits::Repr;

#[pyclass(module = "az.parsing", name = "Statement", frozen, subclass)]
pub(super) struct PyStatement {}

impl PyStatement {
    fn new() -> PyClassInitializer<Self> {
        PyClassInitializer::from(Self {})
    }
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "ExpressionStatement", extends = PyStatement, frozen, get_all)]
pub(super) struct PyExpressionStatement {
    expression: OwnedExpression,
}

#[pymethods]
impl PyExpressionStatement {
    #[new]
    fn new(expression: OwnedExpression) -> PyClassInitializer<Self> {
        PyStatement::new().add_subclass(Self { expression })
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

#[derive(Clone)]
pub(super) enum OwnedStatement {
    Expression(PyExpressionStatement),
}

impl<'a> From<Statement<'a>> for OwnedStatement {
    fn from(value: Statement<'a>) -> Self {
        match value {
            Statement::Expression(expression) => {
                Self::Expression(PyExpressionStatement {
                    expression: expression.into(),
                })
            }
        }
    }
}

impl IntoPy<PyObject> for OwnedStatement {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            OwnedStatement::Expression(wrapped) => {
                Py::new(py, PyExpressionStatement::new(wrapped.expression))
                    .unwrap_or_else(|error| {
                        panic!(
                            "Failed to create {}: {}.",
                            PyExpressionStatement::NAME,
                            error
                        )
                    })
                    .into_py(py)
            }
        }
    }
}

impl Repr for PyExpressionStatement {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!("{}({})", Self::NAME, self.expression.repr(py)?,))
    }
}

impl Repr for OwnedStatement {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            Self::Expression(wrapped) => wrapped.repr(py),
        }
    }
}
