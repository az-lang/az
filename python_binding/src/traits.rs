use pyo3::pyclass::boolean_struct::True;
use pyo3::pyclass::CompareOp;
use pyo3::types::{
    PyAnyMethods, PyString, PyStringMethods, PyType, PyTypeMethods,
};
use pyo3::{
    Bound, Py, PyAny, PyClass, PyObject, PyResult, Python, ToPyObject,
};

pub(crate) trait Repr {
    fn repr(&self, py: Python<'_>) -> PyResult<String>;
}

pub(crate) trait RichCmp {
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

impl<T: ?Sized + Repr> Repr for Box<T> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.as_ref().repr(py)
    }
}

impl<T: PyClass<Frozen = True> + Repr + Sync> Repr for Py<T> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.get().repr(py)
    }
}

impl<T: Repr> Repr for Option<T> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            None => py
                .None()
                .bind(py)
                .repr()
                .and_then(|value| value.to_str().map(Into::into)),
            Some(value) => value.repr(py),
        }
    }
}

impl<'py> Repr for Bound<'py, PyType> {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        self.qualname()
            .and_then(|value| value.to_str().map(Into::into))
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
