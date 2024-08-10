use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::Associativity;

use super::traits::TryToJsString;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[wasm_bindgen(js_name = "Associativity")]
pub enum JsAssociativity {
    LEFT_TO_RIGHT,
    RIGHT_TO_LEFT,
}

impl From<&Associativity> for JsAssociativity {
    fn from(value: &Associativity) -> Self {
        match value {
            Associativity::LeftToRight => Self::LEFT_TO_RIGHT,
            Associativity::RightToLeft => Self::RIGHT_TO_LEFT,
        }
    }
}

impl From<Associativity> for JsAssociativity {
    fn from(value: Associativity) -> Self {
        (&value).into()
    }
}

impl From<JsAssociativity> for Associativity {
    fn from(value: JsAssociativity) -> Self {
        match value {
            JsAssociativity::LEFT_TO_RIGHT => Self::LeftToRight,
            JsAssociativity::RIGHT_TO_LEFT => Self::RightToLeft,
        }
    }
}

impl TryToJsString for Associativity {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        JsAssociativity::from(self).try_to_js_string()
    }
}

impl TryToJsString for JsAssociativity {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "Associativity.{}",
            match self {
                Self::LEFT_TO_RIGHT => "LEFT_TO_RIGHT",
                Self::RIGHT_TO_LEFT => "RIGHT_TO_LEFT",
            }
        ))
    }
}
