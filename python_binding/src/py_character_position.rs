use az::tokenization::CharacterPosition;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[pyclass(
    module = "az.tokenization",
    name = "CharacterPosition",
    frozen,
    get_all
)]
#[derive(Clone)]
pub(super) struct PyCharacterPosition {
    byte: usize,
    utf_8: usize,
}

#[pymethods]
impl PyCharacterPosition {
    #[new]
    fn new(byte: usize, utf_8: usize) -> Self {
        Self { byte, utf_8 }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<CharacterPosition> for PyCharacterPosition {
    fn from(value: CharacterPosition) -> Self {
        let CharacterPosition { byte, utf_8 } = value;
        Self { byte, utf_8 }
    }
}

impl From<&PyCharacterPosition> for CharacterPosition {
    fn from(value: &PyCharacterPosition) -> Self {
        let PyCharacterPosition { byte, utf_8 } = value;
        Self {
            byte: *byte,
            utf_8: *utf_8,
        }
    }
}

impl Repr for PyCharacterPosition {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}(byte={}, utf_8={})",
            Self::NAME,
            self.byte,
            self.utf_8
        ))
    }
}
