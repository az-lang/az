use std::fmt::{Display, Formatter};

use super::constants::{
    F32_NAME, F64_NAME, I16_NAME, I32_NAME, I64_NAME, I8_NAME, ISIZE_NAME,
    U16_NAME, U32_NAME, U64_NAME, U8_NAME, USIZE_NAME,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumericLiteralType {
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    ISize,
    U8,
    U16,
    U32,
    U64,
    USize,
}

impl<'a> From<&'a NumericLiteralType> for &'static str {
    fn from(value: &'a NumericLiteralType) -> Self {
        match value {
            NumericLiteralType::F32 => F32_NAME,
            NumericLiteralType::F64 => F64_NAME,
            NumericLiteralType::I8 => I8_NAME,
            NumericLiteralType::I16 => I16_NAME,
            NumericLiteralType::I32 => I32_NAME,
            NumericLiteralType::I64 => I64_NAME,
            NumericLiteralType::ISize => ISIZE_NAME,
            NumericLiteralType::U8 => U8_NAME,
            NumericLiteralType::U16 => U16_NAME,
            NumericLiteralType::U32 => U32_NAME,
            NumericLiteralType::U64 => U64_NAME,
            NumericLiteralType::USize => USIZE_NAME,
        }
    }
}

impl Display for NumericLiteralType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.into())
    }
}
