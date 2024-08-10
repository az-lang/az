use pyo3::exceptions::PyOverflowError;
use pyo3::types::PyAnyMethods;
use pyo3::{
    pyclass, pymethods, Bound, IntoPy, PyAny, PyObject, PyRef, PyResult,
    PyTypeInfo, Python,
};

use az::tokenization::Utf8Count;

use super::macros::impl_ordered_rich_cmp_for_baseless_py_class;
use super::traits::{Repr, RichCmp};

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
#[pyclass(module = "az.tokenization", name = "Utf8Count", frozen)]
pub(crate) struct PyUtf8Count(usize);

#[pymethods]
impl PyUtf8Count {
    #[classattr]
    const MAX: Self = Self(usize::MAX);

    #[classattr]
    const MIN: Self = Self(usize::MIN);

    #[new]
    #[pyo3(signature = (_value=None, /))]
    fn new(_value: Option<usize>) -> Self {
        Self(_value.unwrap_or_default())
    }

    fn __add__(
        &self,
        other: &Bound<'_, PyAny>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        if let Ok(other) = PyAnyMethods::extract::<PyRef<'_, Self>>(other) {
            match self.0.checked_add(other.0) {
                Some(value) => Ok(Self(value).into_py(py)),
                None => Err(PyOverflowError::new_err(format!(
                    "Sum of {} & {} overflows.",
                    self.repr(py)?,
                    other.repr(py)?
                ))),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __getnewargs__(&self) -> (usize,) {
        (self.0,)
    }

    fn __int__(&self) -> usize {
        self.0
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: pyclass::CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        self.rich_cmp(other, op, py)
    }

    fn __sub__(
        &self,
        other: &Bound<'_, PyAny>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        if let Ok(other) = PyAnyMethods::extract::<PyRef<'_, Self>>(other) {
            match self.0.checked_sub(other.0) {
                Some(value) => Ok(Self(value).into_py(py)),
                None => Err(PyOverflowError::new_err(format!(
                    "Difference of {} & {} overflows.",
                    self.repr(py)?,
                    other.repr(py)?
                ))),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }
}

impl_ordered_rich_cmp_for_baseless_py_class!(PyUtf8Count);

impl From<Utf8Count> for PyUtf8Count {
    fn from(value: Utf8Count) -> Self {
        Self(value.into())
    }
}

impl From<PyUtf8Count> for Utf8Count {
    fn from(value: PyUtf8Count) -> Self {
        value.0.into()
    }
}

impl From<&PyUtf8Count> for Utf8Count {
    fn from(value: &PyUtf8Count) -> Self {
        Self::from(value.0)
    }
}

impl Repr for Utf8Count {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            PyUtf8Count::NAME,
            usize::from(*self).repr(py)?
        ))
    }
}

impl Repr for PyUtf8Count {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!("{}({})", Self::NAME, self.0.repr(py)?))
    }
}
