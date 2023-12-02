use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone, Eq, PartialEq)]
#[pyclass(module = "az.parsing", name = "FillerKind", frozen)]
pub(super) enum PyFillerKind {
    COMMENT_BLOCK,
    COMMENT_LINE,
    NEWLINE,
    WHITESPACE,
}

#[pymethods]
impl PyFillerKind {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl Repr for PyFillerKind {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self {
                Self::COMMENT_BLOCK => "COMMENT_BLOCK",
                Self::COMMENT_LINE => "COMMENT_LINE",
                Self::NEWLINE => "NEWLINE",
                Self::WHITESPACE => "WHITESPACE",
            }
        ))
    }
}
