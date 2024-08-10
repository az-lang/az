use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::NumericLiteral;

use crate::js_filler::JsFiller;
use crate::js_numeric_literal_type::JsNumericLiteralType;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "NumericLiteral")]
pub struct JsNumericLiteral(NumericLiteral<OwnedString>);

impl JsNumericLiteral {
    const JS_NAME: &'static str = "NumericLiteral";
}

#[wasm_bindgen(js_class = NumericLiteral)]
impl JsNumericLiteral {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsNumericLiteral, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        value: String,
        type_: JsNumericLiteralType,
        position: &JsSubstringPosition,
        fillers: &js_sys::Array,
    ) -> Result<JsNumericLiteral, JsValue> {
        Ok(Self(NumericLiteral {
            value: value.into(),
            type_: type_.into(),
            position: position.ref_to(),
            fillers: fillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn fillers(&self) -> Vec<JsFiller> {
        self.0.fillers.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsSubstringPosition {
        self.0.position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn type_(&self) -> JsNumericLiteralType {
        (&self.0.type_).into()
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.0.value.as_ref().into()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsNumericLiteral) -> bool {
        self == other
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        self.try_to_json()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> Result<String, JsValue> {
        self.try_to_js_string()
    }

    #[wasm_bindgen(js_name = "validateContents")]
    pub fn validate_contents(&self) -> Result<(), JsValue> {
        validate_contents(&self.0)
    }

    #[wasm_bindgen(js_name = "validatePositions")]
    pub fn validate_positions(&self) -> Result<(), JsValue> {
        validate_positions(&self.0)
    }
}

impl From<JsNumericLiteral> for NumericLiteral<OwnedString> {
    fn from(value: JsNumericLiteral) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, NumericLiteral<OwnedString>> for JsValue {
    fn from_ref(value: &'rust NumericLiteral<OwnedString>) -> Self {
        JsNumericLiteral(value.clone()).into()
    }
}

impl GetJsName for JsNumericLiteral {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsNumericLiteral {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for NumericLiteral<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {})",
            JsNumericLiteral::JS_NAME,
            self.value.try_to_js_string()?,
            self.type_.try_to_js_string()?,
            self.position.try_to_js_string()?,
            self.fillers.try_to_js_string()?
        ))
    }
}
