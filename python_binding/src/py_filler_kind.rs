use az::parsing::FillerContent;
use pyo3::sync::GILOnceCell;
use pyo3::{pyclass, pymethods, Bound, Py, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[derive(Clone)]
pub(crate) enum FillerKind {
    CommentBlock,
    CommentLine,
    Newline,
    Whitespace,
}

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "FillerKind", frozen)]
pub(crate) struct PyFillerKind(FillerKind);

impl PyFillerKind {
    pub(crate) fn from_rust(value: FillerKind, py: Python<'_>) -> Py<Self> {
        const RUST_VALUES: [FillerKind; 4usize] = [
            FillerKind::CommentBlock,
            FillerKind::CommentLine,
            FillerKind::Newline,
            FillerKind::Whitespace,
        ];
        static PY_VALUES: GILOnceCell<[Py<PyFillerKind>; RUST_VALUES.len()]> =
            GILOnceCell::new();
        PY_VALUES.get_or_init(py, || {
            RUST_VALUES
                .map(|value| Bound::new(py, Self(value)).unwrap().into())
        })[RUST_VALUES
            .iter()
            .position(|candidate| {
                std::mem::discriminant(candidate)
                    == std::mem::discriminant(&value)
            })
            .unwrap()]
        .clone_ref(py)
    }
}

#[pymethods]
impl PyFillerKind {
    #[allow(non_snake_case)]
    #[classattr]
    fn COMMENT_BLOCK(py: Python<'_>) -> Py<Self> {
        Self::from_rust(FillerKind::CommentBlock, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn COMMENT_LINE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(FillerKind::CommentLine, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn NEWLINE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(FillerKind::Newline, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn WHITESPACE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(FillerKind::Whitespace, py)
    }

    fn __reduce__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<PyFillerKind> for FillerKind {
    fn from(value: PyFillerKind) -> Self {
        value.0
    }
}

impl<StringType> From<&FillerContent<StringType>> for FillerKind {
    fn from(value: &FillerContent<StringType>) -> Self {
        match value {
            FillerContent::CommentBlock(_) => Self::CommentBlock,
            FillerContent::CommentLine(_) => Self::CommentLine,
            FillerContent::Newline => Self::Newline,
            FillerContent::Whitespace(_) => Self::Whitespace,
        }
    }
}

impl Repr for FillerKind {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            PyFillerKind::NAME,
            match self {
                Self::CommentBlock => "COMMENT_BLOCK",
                Self::CommentLine => "COMMENT_LINE",
                Self::Newline => "NEWLINE",
                Self::Whitespace => "WHITESPACE",
            }
        ))
    }
}

impl Repr for PyFillerKind {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}
