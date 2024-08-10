use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::UnidirectionalConditional;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::js_block::JsBlock;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "UnidirectionalConditional")]
pub struct JsUnidirectionalConditional(UnidirectionalConditional<OwnedString>);

impl JsUnidirectionalConditional {
    const JS_NAME: &'static str = "UnidirectionalConditional";
}

#[wasm_bindgen(js_class = UnidirectionalConditional)]
impl JsUnidirectionalConditional {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsUnidirectionalConditional, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        antecedent: &JsValue,
        consequent: &JsBlock,
        #[allow(non_snake_case)] openerPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] openerFillers: &js_sys::Array,
    ) -> Result<JsUnidirectionalConditional, JsValue> {
        Ok(Self(UnidirectionalConditional {
            antecedent: Box::new(antecedent.try_ref_to()?),
            consequent: consequent.ref_to(),
            opener_position: openerPosition.ref_to(),
            opener_fillers: openerFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn antecedent(&self) -> JsValue {
        self.0.antecedent.as_ref().ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn consequent(&self) -> JsBlock {
        self.0.consequent.ref_to()
    }

    #[wasm_bindgen(getter = openerFillers)]
    pub fn opener_fillers(&self) -> Vec<JsFiller> {
        self.0.opener_fillers.ref_to()
    }

    #[wasm_bindgen(getter = openerPosition)]
    pub fn opener_position(&self) -> JsSubstringPosition {
        self.0.opener_position.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsUnidirectionalConditional) -> bool {
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

impl From<JsUnidirectionalConditional>
    for UnidirectionalConditional<OwnedString>
{
    fn from(value: JsUnidirectionalConditional) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, UnidirectionalConditional<OwnedString>>
    for JsValue
{
    fn from_ref(value: &'rust UnidirectionalConditional<OwnedString>) -> Self {
        JsUnidirectionalConditional(value.clone()).into()
    }
}

impl GetJsName for JsUnidirectionalConditional {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsUnidirectionalConditional {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for UnidirectionalConditional<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {})",
            JsUnidirectionalConditional::JS_NAME,
            self.antecedent.try_to_js_string()?,
            self.consequent.try_to_js_string()?,
            self.opener_position.try_to_js_string()?,
            self.opener_fillers.try_to_js_string()?,
        ))
    }
}
