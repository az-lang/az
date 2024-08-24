use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::NumericLiteralType;

use super::traits::TryToJsString;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[wasm_bindgen(js_name = "NumericLiteralType")]
pub enum JsNumericLiteralType {
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

impl From<&NumericLiteralType> for JsNumericLiteralType {
    fn from(value: &NumericLiteralType) -> Self {
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

impl From<JsNumericLiteralType> for NumericLiteralType {
    fn from(value: JsNumericLiteralType) -> Self {
        match value {
            JsNumericLiteralType::F32 => Self::F32,
            JsNumericLiteralType::F64 => Self::F64,
            JsNumericLiteralType::I8 => Self::I8,
            JsNumericLiteralType::I16 => Self::I16,
            JsNumericLiteralType::I32 => Self::I32,
            JsNumericLiteralType::I64 => Self::I64,
            JsNumericLiteralType::ISIZE => Self::ISize,
            JsNumericLiteralType::U8 => Self::U8,
            JsNumericLiteralType::U16 => Self::U16,
            JsNumericLiteralType::U32 => Self::U32,
            JsNumericLiteralType::U64 => Self::U64,
            JsNumericLiteralType::USIZE => Self::USize,
        }
    }
}

impl TryToJsString for NumericLiteralType {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        JsNumericLiteralType::from(self).try_to_js_string()
    }
}

impl TryToJsString for JsNumericLiteralType {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "NumericLiteralType.{}",
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
