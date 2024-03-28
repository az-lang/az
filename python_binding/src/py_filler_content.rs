use pyo3::{PyResult, PyTypeInfo, Python};

use az::parsing::FillerContent;

use super::macros::{
    define_baseless_py_dataclass,
    impl_unordered_rich_cmp_for_baseless_py_class,
};
use super::py_filler_kind::PyFillerKind;
use super::traits::Repr;
use super::types::OwnedStr;

define_baseless_py_dataclass!(
    PyFillerContent,
    "az.parsing",
    "FillerContent",
    kind: PyFillerKind,
    string: String,
);
impl_unordered_rich_cmp_for_baseless_py_class!(PyFillerContent);

impl<StringType> From<FillerContent<StringType>> for PyFillerContent
where
    FillerContent<StringType>: ToString,
{
    fn from(value: FillerContent<StringType>) -> Self {
        let kind = to_py_filler_kind(&value);
        Self {
            kind,
            string: value.to_string(),
        }
    }
}

fn to_py_filler_kind<StringType>(
    value: &FillerContent<StringType>,
) -> PyFillerKind {
    match value {
        FillerContent::CommentBlock(_) => PyFillerKind::COMMENT_BLOCK,
        FillerContent::CommentLine(_) => PyFillerKind::COMMENT_LINE,
        FillerContent::Newline => PyFillerKind::NEWLINE,
        FillerContent::Whitespace(_) => PyFillerKind::WHITESPACE,
    }
}

impl<StringType: for<'a> From<&'a str>> From<PyFillerContent>
    for FillerContent<StringType>
{
    fn from(value: PyFillerContent) -> Self {
        match value.kind {
            PyFillerKind::COMMENT_BLOCK => FillerContent::CommentBlock(
                value.string.split_inclusive('\n').map(Into::into).collect(),
            ),
            PyFillerKind::COMMENT_LINE => {
                FillerContent::CommentLine(value.string.as_str().into())
            }
            PyFillerKind::NEWLINE => FillerContent::Newline,
            PyFillerKind::WHITESPACE => {
                FillerContent::Whitespace(value.string.as_str().into())
            }
        }
    }
}

impl Repr for FillerContent<OwnedStr> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            PyFillerContent::NAME,
            to_py_filler_kind(self).repr(py)?,
            self.to_string().repr(py)?
        ))
    }
}
