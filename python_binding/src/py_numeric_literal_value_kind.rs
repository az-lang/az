use pyo3::sync::GILOnceCell;
use pyo3::{pyclass, pymethods, Bound, Py, PyResult, PyTypeInfo, Python};

use az::tokenization::NumericLiteralValueKind;

use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone, Eq, PartialEq)]
#[pyclass(
    module = "az.tokenization",
    name = "NumericLiteralValueKind",
    eq,
    frozen
)]
pub(crate) struct PyNumericLiteralValueKind(NumericLiteralValueKind);

impl PyNumericLiteralValueKind {
    pub(crate) fn from_rust(
        value: NumericLiteralValueKind,
        py: Python<'_>,
    ) -> Py<Self> {
        const RUST_VALUES: [NumericLiteralValueKind; 2usize] = [
            NumericLiteralValueKind::FloatingPoint,
            NumericLiteralValueKind::Integer,
        ];
        static PY_VALUES: GILOnceCell<
            [Py<PyNumericLiteralValueKind>; RUST_VALUES.len()],
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
impl PyNumericLiteralValueKind {
    #[allow(non_snake_case)]
    #[classattr]
    fn FLOATING_POINT(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralValueKind::FloatingPoint, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn INTEGER(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralValueKind::Integer, py)
    }

    fn __reduce__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<PyNumericLiteralValueKind> for NumericLiteralValueKind {
    fn from(value: PyNumericLiteralValueKind) -> Self {
        value.0
    }
}

impl Repr for NumericLiteralValueKind {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            PyNumericLiteralValueKind::NAME,
            match self {
                Self::FloatingPoint => "FLOATING_POINT",
                Self::Integer => "INTEGER",
            }
        ))
    }
}

impl Repr for PyNumericLiteralValueKind {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        self.0.repr(py)
    }
}
