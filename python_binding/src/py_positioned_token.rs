use az::tokenization::PositionedToken;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::py_substring_position::PySubstringPosition;
use super::py_token::PyToken;
use super::traits::Repr;

#[derive(Clone)]
#[pyclass(module = "az.tokenization", name = "PositionedToken")]
pub(super) struct PyPositionedToken {
    #[pyo3(get)]
    position: PySubstringPosition,
    #[pyo3(get)]
    token: PyToken,
}

#[pymethods]
impl PyPositionedToken {
    #[new]
    fn new(position: PySubstringPosition, token: PyToken) -> Self {
        Self { position, token }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl<'a> From<PositionedToken<'a>> for PyPositionedToken {
    fn from(value: PositionedToken<'a>) -> Self {
        Self {
            position: value.position.into(),
            token: value.token.into(),
        }
    }
}

impl<'a> From<&'a PyPositionedToken> for PositionedToken<'a> {
    fn from(value: &'a PyPositionedToken) -> Self {
        Self {
            position: (&value.position).into(),
            token: (&value.token).into(),
        }
    }
}

impl Repr for PyPositionedToken {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}(position={}, token={})",
            Self::NAME,
            self.position.repr(py)?,
            self.token.repr(py)?
        ))
    }
}
