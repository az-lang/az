use pyo3::pyclass::CompareOp;
use pyo3::{pyclass, pymethods, Bound, PyAny, PyObject, PyResult, Python};

use az::parsing::Precedence;

use super::macros::impl_ordered_rich_cmp_for_baseless_py_class;
use super::traits::RichCmp;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
#[pyclass(module = "az.parsing", name = "Precedence", frozen)]
pub(super) struct PyPrecedence(Precedence);

impl_ordered_rich_cmp_for_baseless_py_class!(PyPrecedence);

#[pymethods]
impl PyPrecedence {
    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        self.rich_cmp(other, op, py)
    }
}

impl From<Precedence> for PyPrecedence {
    fn from(value: Precedence) -> Self {
        Self(value)
    }
}

impl From<PyPrecedence> for Precedence {
    fn from(value: PyPrecedence) -> Self {
        value.0
    }
}
