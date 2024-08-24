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
    start: PyCharacterPosition,
    end: PyCharacterPosition,
);

impl_unordered_rich_cmp_for_baseless_py_class!(PySubstringPosition);

impl From<SubstringPosition> for PySubstringPosition {
    fn from(value: SubstringPosition) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl From<PySubstringPosition> for SubstringPosition {
    fn from(value: PySubstringPosition) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl Repr for SubstringPosition {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}(start={}, end={})",
            PySubstringPosition::NAME,
            self.start.repr(py)?,
            self.end.repr(py)?
        ))
    }
}
