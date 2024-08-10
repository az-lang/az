use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::NumericLiteralValueKind;

use super::traits::TryToJsString;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[wasm_bindgen(js_name = "NumericLiteralValueKind")]
pub enum JsNumericLiteralValueKind {
    FLOATING_POINT,
    INTEGER,
}

impl From<&NumericLiteralValueKind> for JsNumericLiteralValueKind {
    fn from(value: &NumericLiteralValueKind) -> Self {
        match value {
            NumericLiteralValueKind::FloatingPoint => Self::FLOATING_POINT,
            NumericLiteralValueKind::Integer => Self::INTEGER,
        }
    }
}

impl From<JsNumericLiteralValueKind> for NumericLiteralValueKind {
    fn from(value: JsNumericLiteralValueKind) -> Self {
        match value {
            JsNumericLiteralValueKind::FLOATING_POINT => Self::FloatingPoint,
            JsNumericLiteralValueKind::INTEGER => Self::Integer,
        }
    }
}

impl TryToJsString for NumericLiteralValueKind {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        JsNumericLiteralValueKind::from(self).try_to_js_string()
    }
}

impl TryToJsString for JsNumericLiteralValueKind {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "NumericLiteralValueKind.{}",
            match self {
                Self::FLOATING_POINT => "FLOATING_POINT",
                Self::INTEGER => "INTEGER",
            }
        ))
    }
}
