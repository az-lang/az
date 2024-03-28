use pyo3::pyclass::CompareOp;
use pyo3::types::{
    PyAnyMethods, PyString, PyStringMethods, PyType, PyTypeMethods,
};
use pyo3::{Bound, PyAny, PyObject, PyResult, Python, ToPyObject};

pub(super) trait Repr {
    fn repr(&self, py: Python<'_>) -> PyResult<String>;
}

pub(super) trait RichCmp {
    fn rich_cmp(
        &self,
        other: &Bound<'_, PyAny>,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyResult<PyObject>;
}

impl Repr for char {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        PyString::new_bound(py, &self.to_string())
            .repr()
            .map(|value| value.to_string())
    }
}

impl Repr for str {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        PyString::new_bound(py, self)
            .repr()
            .map(|value| value.to_string())
    }
}

impl Repr for usize {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.to_object(py)
            .bind(py)
            .repr()
            .map(|value| value.to_string())
    }
}

impl<T: Repr + ?Sized> Repr for Box<T> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.as_ref().repr(py)
    }
}

impl<T: Repr> Repr for Option<T> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            None => py
                .None()
                .bind(py)
                .repr()
                .and_then(|value| value.to_str().map(String::from)),
            Some(value) => value.repr(py),
        }
    }
}

impl<'py> Repr for Bound<'py, PyType> {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        self.qualname().map(Into::into)
    }
}

impl Repr for String {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.as_str().repr(py)
    }
}

impl<T: Repr> Repr for Vec<T> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "[{}]",
            self.iter()
                .map(|element| element.repr(py))
                .collect::<PyResult<Vec<_>>>()?
                .join(", ")
        ))
    }
}
