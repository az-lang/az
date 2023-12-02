use pyo3::{PyResult, PyTypeInfo, Python};

use az::tokenization::SubstringPosition;

use super::macros::{
    define_baseless_py_dataclass,
    impl_unordered_rich_cmp_for_baseless_py_class,
};
use super::py_character_position::PyCharacterPosition;
use super::traits::Repr;

define_baseless_py_dataclass!(
    PySubstringPosition,
    "az.tokenization",
    "SubstringPosition",
    *,
    start_line: usize,
    start_character: PyCharacterPosition,
    end_line: usize,
    end_character: PyCharacterPosition,
);
impl_unordered_rich_cmp_for_baseless_py_class!(PySubstringPosition);

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

impl From<PySubstringPosition> for SubstringPosition {
    fn from(value: PySubstringPosition) -> Self {
        Self {
            start_line: value.start_line,
            start_character: value.start_character.into(),
            end_line: value.end_line,
            end_character: value.end_character.into(),
        }
    }
}

impl Repr for SubstringPosition {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}(start_line={}, start_character={}, end_line={}, end_character={})",
            PySubstringPosition::NAME,
            self.start_line.repr(py)?,
            self.start_character.repr(py)?,
            self.end_line.repr(py)?,
            self.end_character.repr(py)?,
        ))
    }
}
