use pyo3::{
    pyclass, pymethods, Bound, IntoPy, Py, PyAny, PyClassInitializer,
    PyObject, PyResult, PyTypeInfo, Python,
};

use az::parsing::Identifier;

use crate::macros::impl_unordered_rich_cmp_for_derived_py_class;
use crate::py_filler::PyFillers;
use crate::py_substring_position::PySubstringPosition;
use crate::traits::{Repr, RichCmp};
use crate::types::OwnedStr;

use super::py_expression::PyExpression;

#[derive(Clone, PartialEq)]
#[pyclass(
    module = "az.parsing", name = "Identifier", extends = PyExpression, frozen
)]
pub(crate) struct PyIdentifier(Identifier<OwnedStr>);

#[pymethods]
impl PyIdentifier {
    #[getter]
    fn fillers(&self) -> PyFillers {
        self.0.fillers.iter().cloned().map(Into::into).collect()
    }

    #[getter]
    fn position(&self) -> PySubstringPosition {
        self.0.position.clone().into()
    }

    #[getter]
    fn string(&self) -> &str {
        &self.0.string
    }

    #[new]
    #[pyo3(signature = (string, /, *, position, fillers))]
    fn new(
        string: String,
        position: PySubstringPosition,
        fillers: PyFillers,
    ) -> PyClassInitializer<Self> {
        PyExpression::new().add_subclass(Self(Identifier {
            string: string.into(),
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

impl From<Identifier<OwnedStr>> for PyIdentifier {
    fn from(value: Identifier<OwnedStr>) -> Self {
        Self(value)
    }
}

impl From<PyIdentifier> for Identifier<OwnedStr> {
    fn from(value: PyIdentifier) -> Self {
        value.0
    }
}

impl IntoPy<PyObject> for PyIdentifier {
    fn into_py(self, py: Python<'_>) -> PyObject {
        Py::new(py, PyExpression::new().add_subclass(self))
            .unwrap_or_else(|error| {
                panic!("Failed to create {}: {}.", Self::NAME, error)
            })
            .into_py(py)
    }
}

impl Repr for Identifier<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, position={}, fillers={})",
            PyIdentifier::NAME,
            self.string.repr(py)?,
            self.position.repr(py)?,
            self.fillers.repr(py)?,
        ))
    }
}

impl Repr for PyIdentifier {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_derived_py_class!(PyIdentifier, PyExpression);
