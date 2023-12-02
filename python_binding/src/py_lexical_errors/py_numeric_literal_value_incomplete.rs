use pyo3::{
    pyclass, pymethods, Bound, PyAny, PyClassInitializer, PyErr, PyObject,
    PyResult, PyTypeInfo, Python,
};

use az::tokenization::NumericLiteralValueIncomplete;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_numeric_literal_value_kind::PyNumericLiteralValueKind;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::TokenOwnedStr;

use super::py_lexical_error::PyLexicalError;

#[derive(Clone, PartialEq)]
#[pyclass(
module = "az.tokenization", name = "NumericLiteralValueIncomplete",
extends = PyLexicalError, frozen
)]
pub(crate) struct PyNumericLiteralValueIncomplete(
    NumericLiteralValueIncomplete<TokenOwnedStr>,
);

#[pymethods]
impl PyNumericLiteralValueIncomplete {
    #[getter]
    fn kind(&self) -> PyNumericLiteralValueKind {
        self.0.kind.into()
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
    #[pyo3(signature = (kind, position, string, /))]
    fn new(
        kind: PyNumericLiteralValueKind,
        position: PySubstringPosition,
        string: String,
    ) -> PyClassInitializer<Self> {
        PyLexicalError::new().add_subclass(Self(
            NumericLiteralValueIncomplete {
                kind: kind.into(),
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

impl From<NumericLiteralValueIncomplete<TokenOwnedStr>>
    for PyNumericLiteralValueIncomplete
{
    fn from(value: NumericLiteralValueIncomplete<TokenOwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyNumericLiteralValueIncomplete> for PyErr {
    fn from(value: PyNumericLiteralValueIncomplete) -> Self {
        Self::new::<PyNumericLiteralValueIncomplete, _>((
            value.kind(),
            value.position(),
            value.string(),
        ))
    }
}

impl Repr for NumericLiteralValueIncomplete<TokenOwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {})",
            PyNumericLiteralValueIncomplete::NAME,
            self.kind.repr(py)?,
            self.position.repr(py)?,
            self.string.repr(py)?,
        ))
    }
}

impl Repr for PyNumericLiteralValueIncomplete {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(
    PyNumericLiteralValueIncomplete,
    PyLexicalError
);
