use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::tokenization::NumericLiteralTypeSuffixIncomplete;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_numeric_literal_value_kind::PyNumericLiteralValueKind;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedStr;

use super::py_lexical_error::PyLexicalError;

#[derive(Clone, PartialEq)]
#[pyclass(
module = "az.tokenization", name = "NumericLiteralTypeSuffixIncomplete",
extends = PyLexicalError, frozen
)]
pub(crate) struct PyNumericLiteralTypeSuffixIncomplete(
    NumericLiteralTypeSuffixIncomplete<TokenOwnedStr>,
);

#[pymethods]
impl PyNumericLiteralTypeSuffixIncomplete {
    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[getter]
    fn string(&self) -> String {
        self.0.string.as_ref().into()
    }

    #[getter]
    fn value(&self) -> String {
        self.0.value.as_ref().into()
    }

    #[getter]
    fn value_kind(&self) -> PyNumericLiteralValueKind {
        self.0.value_kind.into()
    }

    #[new]
    #[pyo3(signature = (position, string, value, value_kind, /))]
    fn new(
        position: PySubstringPosition,
        string: String,
        value: String,
        value_kind: PyNumericLiteralValueKind,
    ) -> PyClassInitializer<Self> {
        PyLexicalError::new().add_subclass(Self(
            NumericLiteralTypeSuffixIncomplete {
                position: position.into(),
                string: string.into(),
                value: value.into(),
                value_kind: value_kind.into(),
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

impl From<NumericLiteralTypeSuffixIncomplete<TokenOwnedStr>>
    for PyNumericLiteralTypeSuffixIncomplete
{
    fn from(value: NumericLiteralTypeSuffixIncomplete<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyNumericLiteralTypeSuffixIncomplete> for PyErr {
    fn from(value: PyNumericLiteralTypeSuffixIncomplete) -> Self {
        Self::new::<PyNumericLiteralTypeSuffixIncomplete, _>((
            value.position(),
            value.string(),
            value.value(),
            value.value_kind(),
        ))
    }
}

impl Repr for NumericLiteralTypeSuffixIncomplete<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, {})",
            PyNumericLiteralTypeSuffixIncomplete::NAME,
            self.position.repr(py)?,
            self.string.repr(py)?,
            self.value.repr(py)?,
            self.value_kind.repr(py)?,
        ))
    }
}

impl Repr for PyNumericLiteralTypeSuffixIncomplete {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyNumericLiteralTypeSuffixIncomplete,
    PyLexicalError
);
