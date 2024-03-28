use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::tokenization::IdentifierUnexpectedCharacter;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedStr;

use super::py_lexical_error::PyLexicalError;

#[derive(Clone, PartialEq)]
#[pyclass(
module = "az.tokenization", name = "IdentifierUnexpectedCharacter",
extends = PyLexicalError, frozen
)]
pub(crate) struct PyIdentifierUnexpectedCharacter(
    IdentifierUnexpectedCharacter<TokenOwnedStr>,
);

#[pymethods]
impl PyIdentifierUnexpectedCharacter {
    #[getter]
    fn character(&self) -> char {
        self.0.character
    }

    #[getter]
    fn expected(&self) -> String {
        self.0.expected.as_ref().into()
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
    #[pyo3(signature = (character, expected, position, string, /))]
    fn new(
        character: char,
        expected: String,
        position: PySubstringPosition,
        string: String,
    ) -> PyClassInitializer<Self> {
        PyLexicalError::new().add_subclass(Self(
            IdentifierUnexpectedCharacter {
                character,
                expected: expected.into(),
                position: position.into(),
                string: string.into(),
            },
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

impl From<IdentifierUnexpectedCharacter<TokenOwnedStr>>
    for PyIdentifierUnexpectedCharacter
{
    fn from(value: IdentifierUnexpectedCharacter<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyIdentifierUnexpectedCharacter> for PyErr {
    fn from(value: PyIdentifierUnexpectedCharacter) -> Self {
        Self::new::<PyIdentifierUnexpectedCharacter, _>((
            value.character(),
            value.expected(),
            value.position(),
            value.string(),
        ))
    }
}

impl Repr for IdentifierUnexpectedCharacter<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, {})",
            PyIdentifierUnexpectedCharacter::NAME,
            self.character.repr(py)?,
            self.expected.repr(py)?,
            self.position.repr(py)?,
            self.string.repr(py)?,
        ))
    }
}

impl Repr for PyIdentifierUnexpectedCharacter {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyIdentifierUnexpectedCharacter,
    PyLexicalError
);
