use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use az::parsing::Associativity;

use super::macros::impl_unordered_rich_cmp_for_baseless_py_class;
use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone, Eq, Hash, PartialEq)]
#[pyclass(module = "az.parsing", name = "Associativity", frozen)]
pub(super) enum PyAssociativity {
    LEFT_TO_RIGHT,
    RIGHT_TO_LEFT,
}

impl_unordered_rich_cmp_for_baseless_py_class!(PyAssociativity);

#[pymethods]
impl PyAssociativity {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<Associativity> for PyAssociativity {
    fn from(value: Associativity) -> Self {
        match value {
            Associativity::LeftToRight => Self::LEFT_TO_RIGHT,
            Associativity::RightToLeft => Self::RIGHT_TO_LEFT,
        }
    }
}

impl From<PyAssociativity> for Associativity {
    fn from(value: PyAssociativity) -> Self {
        match value {
            PyAssociativity::LEFT_TO_RIGHT => Associativity::LeftToRight,
            PyAssociativity::RIGHT_TO_LEFT => Associativity::RightToLeft,
        }
    }
}

impl Repr for PyAssociativity {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self {
                Self::LEFT_TO_RIGHT => "LEFT_TO_RIGHT",
                Self::RIGHT_TO_LEFT => "RIGHT_TO_LEFT",
            }
        ))
    }
}
