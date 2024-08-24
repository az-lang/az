use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::BidirectionalConditional;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::js_block::JsBlock;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "BidirectionalConditional")]
pub struct JsBidirectionalConditional(BidirectionalConditional<OwnedString>);

impl JsBidirectionalConditional {
    const JS_NAME: &'static str = "BidirectionalConditional";
}

#[wasm_bindgen(js_class = BidirectionalConditional)]
impl JsBidirectionalConditional {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsBidirectionalConditional, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        antecedent: &JsValue,
        consequent: &JsBlock,
        alternative: &JsValue,
        #[allow(non_snake_case)]
        antecedentOpenerPosition: &JsSubstringPosition,
        #[allow(non_snake_case)]
        alternativeOpenerPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] antecedentOpenerFillers: &js_sys::Array,
        #[allow(non_snake_case)] alternativeOpenerFillers: &js_sys::Array,
    ) -> Result<JsBidirectionalConditional, JsValue> {
        Ok(Self(BidirectionalConditional {
            antecedent: Box::new(antecedent.try_ref_to()?),
            consequent: consequent.ref_to(),
            alternative: Box::new(alternative.try_ref_to()?),
            antecedent_opener_position: antecedentOpenerPosition.ref_to(),
            alternative_opener_position: alternativeOpenerPosition.ref_to(),
            antecedent_opener_fillers: antecedentOpenerFillers.try_ref_to()?,
            alternative_opener_fillers: alternativeOpenerFillers
                .try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn alternative(&self) -> JsValue {
        self.0.alternative.as_ref().ref_to()
    }

    #[wasm_bindgen(getter = alternativeOpenerFillers)]
    pub fn alternative_opener_fillers(&self) -> Vec<JsFiller> {
        self.0.alternative_opener_fillers.ref_to()
    }

    #[wasm_bindgen(getter = alternativeOpenerPosition)]
    pub fn alternative_opener_position(&self) -> JsSubstringPosition {
        self.0.alternative_opener_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn antecedent(&self) -> JsValue {
        self.0.antecedent.as_ref().ref_to()
    }

    #[wasm_bindgen(getter = antecedentOpenerFillers)]
    pub fn antecedent_opener_fillers(&self) -> Vec<JsFiller> {
        self.0.antecedent_opener_fillers.ref_to()
    }

    #[wasm_bindgen(getter = antecedentOpenerPosition)]
    pub fn antecedent_opener_position(&self) -> JsSubstringPosition {
        self.0.antecedent_opener_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn consequent(&self) -> JsBlock {
        self.0.consequent.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsBidirectionalConditional) -> bool {
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

impl From<JsBidirectionalConditional>
    for BidirectionalConditional<OwnedString>
{
    fn from(value: JsBidirectionalConditional) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, BidirectionalConditional<OwnedString>> for JsValue {
    fn from_ref(value: &'rust BidirectionalConditional<OwnedString>) -> Self {
        JsBidirectionalConditional(value.clone()).into()
    }
}

impl GetJsName for JsBidirectionalConditional {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsBidirectionalConditional {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for BidirectionalConditional<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {}, {}, {})",
            JsBidirectionalConditional::JS_NAME,
            self.antecedent.try_to_js_string()?,
            self.consequent.try_to_js_string()?,
            self.alternative.try_to_js_string()?,
            self.antecedent_opener_position.try_to_js_string()?,
            self.alternative_opener_position.try_to_js_string()?,
            self.antecedent_opener_fillers.try_to_js_string()?,
            self.alternative_opener_fillers.try_to_js_string()?,
        ))
    }
}
