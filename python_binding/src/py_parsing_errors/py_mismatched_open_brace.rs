use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::parsing::MismatchedOpenBrace;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};

use super::py_parsing_error::PyParsingError;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "MismatchedOpenBrace",
    extends = PyParsingError, frozen
)]
pub(crate) struct PyMismatchedOpenBrace(MismatchedOpenBrace);

#[pymethods]
impl PyMismatchedOpenBrace {
    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[new]
    #[pyo3(signature = (position, /))]
    fn new(position: PySubstringPosition) -> PyClassInitializer<Self> {
        PyParsingError::new().add_subclass(Self(MismatchedOpenBrace {
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

impl From<MismatchedOpenBrace> for PyMismatchedOpenBrace {
    fn from(value: MismatchedOpenBrace) -> Self {
        Self(value)
    }
}

impl From<PyMismatchedOpenBrace> for PyErr {
    fn from(value: PyMismatchedOpenBrace) -> Self {
        Self::new::<PyMismatchedOpenBrace, _>((value.position(),))
    }
}

impl Repr for MismatchedOpenBrace {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            PyMismatchedOpenBrace::NAME,
            self.position.repr(py)?
        ))
    }
}

impl Repr for PyMismatchedOpenBrace {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyMismatchedOpenBrace,
    PyParsingError
);
