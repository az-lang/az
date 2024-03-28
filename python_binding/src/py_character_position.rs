use pyo3::{PyResult, PyTypeInfo, Python};

use az::tokenization::CharacterPosition;

use super::macros::{
    define_baseless_py_dataclass,
    impl_unordered_rich_cmp_for_baseless_py_class,
};
use super::py_byte_index::PyByteIndex;
use super::py_utf8_index::PyUtf8Index;
use super::traits::Repr;

define_baseless_py_dataclass!(
    PyCharacterPosition,
    "az.tokenization",
    "CharacterPosition",
    *,
    byte: PyByteIndex,
    utf_8: PyUtf8Index,
);
impl_unordered_rich_cmp_for_baseless_py_class!(PyCharacterPosition);

impl From<CharacterPosition> for PyCharacterPosition {
    fn from(value: CharacterPosition) -> Self {
        let CharacterPosition { byte, utf_8 } = value;
        Self {
            byte: byte.into(),
            utf_8: utf_8.into(),
        }
    }
}

impl From<PyCharacterPosition> for CharacterPosition {
    fn from(value: PyCharacterPosition) -> Self {
        let PyCharacterPosition { byte, utf_8 } = value;
        Self {
            byte: byte.into(),
            utf_8: utf_8.into(),
        }
    }
}

impl Repr for CharacterPosition {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}(byte={}, utf_8={})",
            PyCharacterPosition::NAME,
            self.byte.repr(py)?,
            self.utf_8.repr(py)?
        ))
    }
}
