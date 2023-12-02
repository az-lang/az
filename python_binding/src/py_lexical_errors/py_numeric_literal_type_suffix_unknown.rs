use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::tokenization::NumericLiteralTypeSuffixUnknown;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_numeric_literal_value_kind::PyNumericLiteralValueKind;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedStr;

use super::py_lexical_error::PyLexicalError;

#[derive(Clone, PartialEq)]
#[pyclass(
module = "az.tokenization", name = "NumericLiteralTypeSuffixUnknown",
extends = PyLexicalError, frozen
)]
pub(crate) struct PyNumericLiteralTypeSuffixUnknown(
    NumericLiteralTypeSuffixUnknown<TokenOwnedStr>,
);

#[pymethods]
impl PyNumericLiteralTypeSuffixUnknown {
    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[getter]
    fn string(&self) -> String {
        self.0.string.as_ref().into()
    }

    #[getter]
    fn type_suffix(&self) -> String {
        self.0.type_suffix.as_ref().into()
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
    #[pyo3(signature = (position, string, type_suffix, value, value_kind, /))]
    fn new(
        position: PySubstringPosition,
        string: String,
        type_suffix: String,
        value: String,
        value_kind: PyNumericLiteralValueKind,
    ) -> PyClassInitializer<Self> {
        PyLexicalError::new().add_subclass(Self(
            NumericLiteralTypeSuffixUnknown {
                position: position.into(),
                string: string.into(),
                type_suffix: type_suffix.into(),
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

impl From<NumericLiteralTypeSuffixUnknown<TokenOwnedStr>>
    for PyNumericLiteralTypeSuffixUnknown
{
    fn from(value: NumericLiteralTypeSuffixUnknown<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyNumericLiteralTypeSuffixUnknown> for PyErr {
    fn from(value: PyNumericLiteralTypeSuffixUnknown) -> Self {
        Self::new::<PyNumericLiteralTypeSuffixUnknown, _>((
            value.position(),
            value.string(),
            value.type_suffix(),
            value.value(),
            value.value_kind(),
        ))
    }
}

impl Repr for NumericLiteralTypeSuffixUnknown<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, {}, {})",
            PyNumericLiteralTypeSuffixUnknown::NAME,
            self.position.repr(py)?,
            self.string.repr(py)?,
            self.type_suffix.repr(py)?,
            self.value.repr(py)?,
            self.value_kind.repr(py)?
        ))
    }
}

impl Repr for PyNumericLiteralTypeSuffixUnknown {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyNumericLiteralTypeSuffixUnknown,
    PyLexicalError
);
