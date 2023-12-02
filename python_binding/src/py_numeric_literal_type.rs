use az::tokenization::NumericLiteralType;
use pyo3::{pyclass, pymethods, PyResult, PyTypeInfo, Python};

use super::traits::Repr;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[pyclass(module = "az.parsing", name = "NumericLiteralType", frozen)]
pub(super) enum PyNumericLiteralType {
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    ISIZE,
    U8,
    U16,
    U32,
    U64,
    USIZE,
}

#[pymethods]
impl PyNumericLiteralType {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        self.repr(py)
    }
}

impl From<NumericLiteralType> for PyNumericLiteralType {
    fn from(value: NumericLiteralType) -> Self {
        match value {
            NumericLiteralType::F32 => Self::F32,
            NumericLiteralType::F64 => Self::F64,
            NumericLiteralType::I8 => Self::I8,
            NumericLiteralType::I16 => Self::I16,
            NumericLiteralType::I32 => Self::I32,
            NumericLiteralType::I64 => Self::I64,
            NumericLiteralType::ISize => Self::ISIZE,
            NumericLiteralType::U8 => Self::U8,
            NumericLiteralType::U16 => Self::U16,
            NumericLiteralType::U32 => Self::U32,
            NumericLiteralType::U64 => Self::U64,
            NumericLiteralType::USize => Self::USIZE,
        }
    }
}

impl Repr for PyNumericLiteralType {
    fn repr(&self, _py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "{}.{}",
            Self::NAME,
            match self {
                Self::F32 => "F32",
                Self::F64 => "F64",
                Self::I8 => "I8",
                Self::I16 => "I16",
                Self::I32 => "I32",
                Self::I64 => "I64",
                Self::ISIZE => "ISIZE",
                Self::U8 => "U8",
                Self::U16 => "U16",
                Self::U32 => "U32",
                Self::U64 => "U64",
                Self::USIZE => "USIZE",
            }
        ))
    }
}
