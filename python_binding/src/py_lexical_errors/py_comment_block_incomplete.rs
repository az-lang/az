use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::tokenization::CommentBlockIncomplete;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedString;

use super::py_lexical_error::PyLexicalError;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.tokenization", name = "CommentBlockIncomplete",
    extends = PyLexicalError, frozen
)]
pub(crate) struct PyCommentBlockIncomplete(
    CommentBlockIncomplete<TokenOwnedString>,
);

#[pymethods]
impl PyCommentBlockIncomplete {
    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[getter]
    fn string(&self) -> String {
        self.0.string.as_ref().into()
    }

    #[new]
    #[pyo3(signature = (position, string, /))]
    fn new(
        position: PySubstringPosition,
        string: String,
    ) -> PyClassInitializer<Self> {
        PyLexicalError::new().add_subclass(Self(CommentBlockIncomplete {
            string: string.into(),
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

impl From<CommentBlockIncomplete<TokenOwnedString>>
    for PyCommentBlockIncomplete
{
    fn from(value: CommentBlockIncomplete<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyCommentBlockIncomplete> for PyErr {
    fn from(value: PyCommentBlockIncomplete) -> Self {
        Self::new::<PyCommentBlockIncomplete, _>((
            value.position(),
            value.string(),
        ))
    }
}

impl Repr for CommentBlockIncomplete<TokenOwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            PyCommentBlockIncomplete::NAME,
            self.position.repr(py)?,
            self.string.repr(py)?,
        ))
    }
}

impl Repr for PyCommentBlockIncomplete {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyCommentBlockIncomplete,
    PyLexicalError
);
