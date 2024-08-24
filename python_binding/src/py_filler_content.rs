use pyo3::exceptions::{PyBaseExceptionGroup, PyValueError};
use pyo3::pyclass::CompareOp;
use pyo3::{
    pyclass, pymethods, Bound, Py, PyAny, PyErr, PyObject, PyResult,
    PyTypeInfo, Python,
};

use az::parsing::FillerContent;

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::py_filler_kind::{FillerKind, PyFillerKind};
use super::py_validation_errors::PyContentsValidationError;
use super::traits::{Repr, RichCmp};
use super::types::OwnedString;

#[derive(Clone, PartialEq)]
#[pyclass(module = "az.parsing", name = "FillerContent", frozen)]
pub(crate) struct PyFillerContent(FillerContent<OwnedString>);

#[pymethods]
impl PyFillerContent {
    #[new]
    #[pyo3(signature = (kind, state=None, /))]
    fn new(
        kind: PyFillerKind,
        state: Option<String>,
        py: Python<'_>,
    ) -> PyResult<Self> {
        match (kind.into(), state) {
            (FillerKind::CommentBlock, Some(state)) => {
                Ok(Self(FillerContent::CommentBlock(state.into())))
            }
            (FillerKind::CommentLine, Some(state)) => {
                Ok(Self(FillerContent::CommentLine(state.into())))
            }
            (FillerKind::Newline, None) => Ok(Self(FillerContent::Newline)),
            (FillerKind::Whitespace, Some(state)) => {
                Ok(Self(FillerContent::Whitespace(state.into())))
            }
            (kind, state) => Err(PyValueError::new_err(format!(
                "Invalid arguments for {} with {} kind: {}, but got {}.",
                Self::NAME,
                kind.repr(py)?,
                match state {
                    Some(_) => "does not need a state",
                    None => "needs a state",
                },
                state.repr(py)?
            ))),
        }
    }

    #[getter]
    fn kind(&self, py: Python<'_>) -> Py<PyFillerKind> {
        PyFillerKind::from_rust(FillerKind::from(&self.0), py)
    }

    #[getter]
    fn state(&self) -> Option<String> {
        match &self.0 {
            FillerContent::Newline => None,
            FillerContent::CommentBlock(_)
            | FillerContent::CommentLine(_)
            | FillerContent::Whitespace(_) => Some(self.0.to_string()),
        }
    }

    fn validate(&self) -> PyResult<()> {
        self.0.validate().map_err(|errors| {
            PyBaseExceptionGroup::new_err((
                format!("Failed to validate {}.", PyFillerContent::NAME),
                errors
                    .into_iter()
                    .map(|error| {
                        PyErr::new::<PyContentsValidationError, _>((
                            error.to_string(),
                        ))
                    })
                    .collect::<Vec<_>>(),
            ))
        })
    }

    fn __getnewargs__(
        &self,
        py: Python<'_>,
    ) -> (Py<PyFillerKind>, Option<String>) {
        (self.kind(py), self.state())
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        self.rich_cmp(other, op, py)
    }
}

impl From<FillerContent<OwnedString>> for PyFillerContent {
    fn from(value: FillerContent<OwnedString>) -> Self {
        Self(value)
    }
}

impl From<PyFillerContent> for FillerContent<OwnedString> {
    fn from(value: PyFillerContent) -> Self {
        value.0
    }
}

impl Repr for FillerContent<OwnedString> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(match self {
            FillerContent::Newline => {
                format!(
                    "{}({})",
                    PyFillerContent::NAME,
                    FillerKind::from(self).repr(py)?
                )
            }
            FillerContent::CommentBlock(_)
            | FillerContent::CommentLine(_)
            | FillerContent::Whitespace(_) => format!(
                "{}({}, {})",
                PyFillerContent::NAME,
                FillerKind::from(self).repr(py)?,
                self.to_string().repr(py)?
            ),
        })
    }
}

impl Repr for PyFillerContent {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyFillerContent);
