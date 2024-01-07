use pyo3::types::PyString;
use pyo3::{PyResult, Python};

pub(super) trait Repr {
    fn repr(&self, py: Python<'_>) -> PyResult<String>;
}

impl Repr for char {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        PyString::new(py, &self.to_string())
            .repr()
            .map(ToString::to_string)
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

impl<T: Repr> Repr for Option<T> {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        match self {
            None => Ok("None".to_string()),
            Some(value) => value.repr(py),
        }
    }
}

impl Repr for str {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        PyString::new(py, self).repr().map(ToString::to_string)
    }
}

impl Repr for String {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.as_str().repr(py)
    }
}
