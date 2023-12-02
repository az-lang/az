use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::NumericLiteral;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::PyFillers;
use crate::py_numeric_literal_type::PyNumericLiteralType;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "NumericLiteral", extends = PyExpression,
    frozen
)]
pub(crate) struct PyNumericLiteral(NumericLiteral<OwnedStr>);

#[pymethods]
impl PyNumericLiteral {
    #[getter]
    fn fillers(&self) -> PyFillers {
        self.0.fillers.iter().cloned().map(Into::into).collect()
    }

    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[getter]
    fn type_(&self) -> PyNumericLiteralType {
        self.0.type_.into()
    }

    #[getter]
    fn value(&self) -> &str {
        &self.0.value
    }

    #[new]
    #[pyo3(signature = (value, type_, /, *, position, fillers))]
    fn new(
        value: String,
        type_: PyNumericLiteralType,
        position: PySubstringPosition,
        fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(NumericLiteral {
            value: value.into(),
            type_: type_.into(),
            position: position.into(),
            fillers: fillers.into_iter().map(Into::into).collect(),
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

impl From<NumericLiteral<OwnedStr>> for PyNumericLiteral {
    fn from(value: NumericLiteral<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyNumericLiteral> for NumericLiteral<OwnedStr> {
    fn from(value: PyNumericLiteral) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyNumericLiteral {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for NumericLiteral<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, position={}, fillers={})",
            PyNumericLiteral::NAME,
            self.value.repr(py)?,
            self.type_.repr(py)?,
            self.position.repr(py)?,
            self.fillers.repr(py)?,
        ))
    }
}

impl Repr for PyNumericLiteral {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyNumericLiteral, PyExpression);
