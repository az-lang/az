use pyo3::{PyResult, PyTypeInfo, Python};

use az::parsing::{Filler, FillerContent};

use super::macros::{
    define_baseless_py_dataclass,
    impl_unordered_rich_cmp_for_baseless_py_class,
};
use super::py_filler_content::PyFillerContent;
use super::py_substring_position::PySubstringPosition;
use super::traits::Repr;
use super::types::OwnedStr;

define_baseless_py_dataclass!(
    PyFiller,
    "az.parsing",
    "Filler",
    *,
    content: PyFillerContent,
    position: PySubstringPosition,
);
impl_unordered_rich_cmp_for_baseless_py_class!(PyFiller);

pub(super) type PyFillers = Vec<PyFiller>;

impl<StringType> From<Filler<StringType>> for PyFiller
where
    FillerContent<StringType>: Into<PyFillerContent>,
{
    fn from(value: Filler<StringType>) -> Self {
        Self {
            content: value.content.into(),
            position: value.position.into(),
        }
    }
}

impl<StringType> From<PyFiller> for Filler<StringType>
where
    PyFillerContent: Into<FillerContent<StringType>>,
{
    fn from(value: PyFiller) -> Self {
        Self {
            content: value.content.into(),
            position: value.position.into(),
        }
    }
}

impl Repr for Filler<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}(content={}, position={})",
            PyFiller::NAME,
            self.content.repr(py)?,
            self.position.repr(py)?
        ))
    }
}
