use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::tokenization::UnexpectedCharacter;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedStr;

use super::py_lexical_error::PyLexicalError;

#[derive(Clone, PartialEq)]
#[pyclass(
module = "az.tokenization", name = "UnexpectedCharacter",
extends = PyLexicalError, frozen
)]
pub(crate) struct PyUnexpectedCharacter(UnexpectedCharacter<TokenOwnedStr>);

#[pymethods]
impl PyUnexpectedCharacter {
    #[getter]
    fn character(&self) -> char {
        self.0.character
    }

    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[getter]
    fn string(&self) -> String {
        self.0.string.as_ref().into()
    }

    #[new]
    #[pyo3(signature = (character, position, string, /))]
    fn new(
        character: char,
        position: PySubstringPosition,
        string: String,
    ) -> PyClassInitializer<Self> {
        PyLexicalError::new().add_subclass(Self(UnexpectedCharacter {
            character,
            position: position.into(),
            string: string.into(),
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

impl From<UnexpectedCharacter<TokenOwnedStr>> for PyUnexpectedCharacter {
    fn from(value: UnexpectedCharacter<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyUnexpectedCharacter> for PyErr {
    fn from(value: PyUnexpectedCharacter) -> Self {
        Self::new::<PyUnexpectedCharacter, _>((
            value.character(),
            value.position(),
            value.string(),
        ))
    }
}

impl Repr for UnexpectedCharacter<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {})",
            PyUnexpectedCharacter::NAME,
            self.character.repr(py)?,
            self.position.repr(py)?,
            self.string.repr(py)?,
        ))
    }
}

impl Repr for PyUnexpectedCharacter {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyUnexpectedCharacter,
    PyLexicalError
);
