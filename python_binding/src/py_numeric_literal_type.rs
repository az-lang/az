use az::tokenization::NumericLiteralType;
use pyo3::sync::GILOnceCell;
use pyo3::{pyclass, pymethods, Bound, Py, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "NumericLiteralType", frozen)]
pub(crate) struct PyNumericLiteralType(NumericLiteralType);

impl PyNumericLiteralType {
    pub(crate) fn from_rust(
        value: NumericLiteralType,
        py: Python<'_>,
    ) -> Py<Self> {
        const RUST_VALUES: [NumericLiteralType; 12usize] = [
            NumericLiteralType::F32,
            NumericLiteralType::F64,
            NumericLiteralType::I8,
            NumericLiteralType::I16,
            NumericLiteralType::I32,
            NumericLiteralType::I64,
            NumericLiteralType::ISize,
            NumericLiteralType::U8,
            NumericLiteralType::U16,
            NumericLiteralType::U32,
            NumericLiteralType::U64,
            NumericLiteralType::USize,
        ];
        static PY_VALUES: GILOnceCell<
            [Py<PyNumericLiteralType>; RUST_VALUES.len()],
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

impl From<PyNumericLiteralType> for NumericLiteralType {
    fn from(value: PyNumericLiteralType) -> Self {
        value.0
    }
}

#[pymethods]
impl PyNumericLiteralType {
    #[allow(non_snake_case)]
    #[classattr]
    fn F32(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::F32, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn F64(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::F64, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn I8(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::I8, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn I16(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::I16, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn I32(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::I32, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn I64(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::I64, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ISIZE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::ISize, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn U8(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::U8, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn U16(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::U16, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn U32(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::U32, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn U64(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::U64, py)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn USIZE(py: Python<'_>) -> Py<Self> {
        Self::from_rust(NumericLiteralType::USize, py)
    }

    fn __reduce__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl Repr for NumericLiteralType {
    fn repr(&self, py: Python<'_>) -> PyResult<String> {
        PyNumericLiteralType::from_rust(*self, py).repr(py)
    }
}

impl Repr for PyNumericLiteralType {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self.0 {
                NumericLiteralType::F32 => "F32",
                NumericLiteralType::F64 => "F64",
                NumericLiteralType::I8 => "I8",
                NumericLiteralType::I16 => "I16",
                NumericLiteralType::I32 => "I32",
                NumericLiteralType::I64 => "I64",
                NumericLiteralType::ISize => "ISIZE",
                NumericLiteralType::U8 => "U8",
                NumericLiteralType::U16 => "U16",
                NumericLiteralType::U32 => "U32",
                NumericLiteralType::U64 => "U64",
                NumericLiteralType::USize => "USIZE",
            }
        ))
    }
}
