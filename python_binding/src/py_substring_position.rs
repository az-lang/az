use az::tokenization::SubstringPosition;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::py_character_position::PyCharacterPosition;
use super::traits::Repr;

#[pyclass(
    module = "az.tokenization",
    name = "SubstringPosition",
    frozen,
    get_all
)]
#[derive(Clone)]
pub(super) struct PySubstringPosition {
    start_line: usize,
    start_character: PyCharacterPosition,
    end_line: usize,
    end_character: PyCharacterPosition,
}

#[pymethods]
impl PySubstringPosition {
    #[new]
    fn new(
        start_line: usize,
        start_character: PyCharacterPosition,
        end_line: usize,
        end_character: PyCharacterPosition,
    ) -> Self {
        Self {
            start_line,
            start_character,
            end_line,
            end_character,
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<SubstringPosition> for PySubstringPosition {
    fn from(value: SubstringPosition) -> Self {
        let SubstringPosition {
            start_line,
            start_character,
            end_line,
            end_character,
        } = value;
        Self {
            start_line,
            start_character: start_character.into(),
            end_line,
            end_character: end_character.into(),
        }
    }
}

impl From<&PySubstringPosition> for SubstringPosition {
    fn from(value: &PySubstringPosition) -> Self {
        let PySubstringPosition {
            start_line,
            start_character,
            end_line,
            end_character,
        } = value;
        Self {
            start_line: *start_line,
            start_character: start_character.into(),
            end_line: *end_line,
            end_character: end_character.into(),
        }
    }
}

impl Repr for PySubstringPosition {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}(start_line={}, start_character={}, end_line={}, end_character={})",
            Self::NAME,
            self.start_line,
            self.start_character.repr(py)?,
            self.end_line,
            self.end_character.repr(py)?
        ))
    }
}
