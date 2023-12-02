use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::parsing::MismatchedOpenParenthesis;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};

use super::py_parsing_error::PyParsingError;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "MismatchedOpenParenthesis",
    extends = PyParsingError, frozen
)]
pub(crate) struct PyMismatchedOpenParenthesis(MismatchedOpenParenthesis);

#[pymethods]
impl PyMismatchedOpenParenthesis {
    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[new]
    #[pyo3(signature = (position, /))]
    fn new(position: PySubstringPosition) -> PyClassInitializer<Self> {
        PyParsingError::new().add_subclass(Self(MismatchedOpenParenthesis {
            position: position.into(),
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

impl From<MismatchedOpenParenthesis> for PyMismatchedOpenParenthesis {
    fn from(value: MismatchedOpenParenthesis) -> Self {
        Self(value)
    }
}

impl From<PyMismatchedOpenParenthesis> for PyErr {
    fn from(value: PyMismatchedOpenParenthesis) -> Self {
        Self::new::<PyMismatchedOpenParenthesis, _>((value.position(),))
    }
}

impl Repr for MismatchedOpenParenthesis {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            PyMismatchedOpenParenthesis::NAME,
            self.position.repr(py)?
        ))
    }
}

impl Repr for PyMismatchedOpenParenthesis {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyMismatchedOpenParenthesis,
    PyParsingError
);
