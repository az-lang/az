use pyo3::sync::GILOnceCell;
use pyo3::{pyclass, pymethods, Bound, Py, PyResult, PyTypeInfo, Python};

use az::parsing::Associativity;

use super::traits::Repr;

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "Associativity", frozen)]
pub(crate) struct PyAssociativity(Associativity);

impl PyAssociativity {
    pub(crate) fn from_rust(value: Associativity, py: Python<'_>) -> Py<Self> {
        const RUST_VALUES: [Associativity; 2usize] =
            [Associativity::LeftToRight, Associativity::RightToLeft];
        static PY_VALUES: GILOnceCell<
            [Py<PyAssociativity>; RUST_VALUES.len()],
        > = GILOnceCell::new();
        PY_VALUES.get_or_init(py, || {
            RUST_VALUES
                .map(|value| Bound::new(py, Self(value)).unwrap().into())
        })[RUST_VALUES
            .iter()
            .position(|candidate| {
                std::mem::discriminant(candidate)
                    == std::mem::discriminant(&value)
            })
            .unwrap()]
        .clone_ref(py)
    }
}

#[pymethods]
impl PyAssociativity {
    #[allow(non_snake_case)]
    #[classattr]
    fn LEFT_TO_RIGHT(py: Python<'_>) -> Py<Self> {
        Self::from_rust(Associativity::LeftToRight, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn RIGHT_TO_LEFT(py: Python<'_>) -> Py<Self> {
        Self::from_rust(Associativity::RightToLeft, py)
    }

    fn __reduce__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<PyAssociativity> for Associativity {
    fn from(value: PyAssociativity) -> Self {
        value.0
    }
}

impl Repr for Associativity {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            PyAssociativity::NAME,
            match self {
                Self::LeftToRight => "LEFT_TO_RIGHT",
                Self::RightToLeft => "RIGHT_TO_LEFT",
            }
        ))
    }
}

impl Repr for PyAssociativity {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}
