use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::parsing::UnexpectedToken;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_token::PyToken;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedStr;

use super::py_parsing_error::PyParsingError;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "UnexpectedToken",
    extends = PyParsingError, frozen
)]
pub(crate) struct PyUnexpectedToken(UnexpectedToken<TokenOwnedStr>);

#[pymethods]
impl PyUnexpectedToken {
    #[getter]
    fn token(&self) -> PyToken {
        self.0.token.clone().into()
    }

    #[new]
    #[pyo3(signature = (token, /))]
    fn new(token: PyToken) -> PyClassInitializer<Self> {
        PyParsingError::new().add_subclass(Self(UnexpectedToken {
            token: token.into(),
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

impl From<PyUnexpectedToken> for PyErr {
    fn from(value: PyUnexpectedToken) -> Self {
        Self::new::<PyUnexpectedToken, _>((value.token(),))
    }
}

impl From<UnexpectedToken<TokenOwnedStr>> for PyUnexpectedToken {
    fn from(value: UnexpectedToken<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl Repr for UnexpectedToken<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            PyUnexpectedToken::NAME,
            self.token.repr(py)?
        ))
    }
}

impl Repr for PyUnexpectedToken {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyUnexpectedToken,
    PyParsingError
);
