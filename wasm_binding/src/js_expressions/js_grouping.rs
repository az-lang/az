use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::Grouping;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Grouping")]
pub struct JsGrouping(Grouping<OwnedString>);

impl JsGrouping {
    const JS_NAME: &'static str = "Grouping";
}

#[wasm_bindgen(js_class = Grouping)]
impl JsGrouping {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsGrouping, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        expression: &JsValue,
        #[allow(non_snake_case)] openParenthesisPosition: &JsSubstringPosition,
        #[allow(non_snake_case)]
        closeParenthesisPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] openParenthesisFillers: &js_sys::Array,
        #[allow(non_snake_case)] closeParenthesisFillers: &js_sys::Array,
    ) -> Result<JsGrouping, JsValue> {
        Ok(Self(Grouping {
            expression: Box::new(expression.try_ref_to()?),
            open_parenthesis_position: openParenthesisPosition.ref_to(),
            close_parenthesis_position: closeParenthesisPosition.ref_to(),
            open_parenthesis_fillers: openParenthesisFillers.try_ref_to()?,
            close_parenthesis_fillers: closeParenthesisFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter = closeParenthesisFillers)]
    pub fn close_parenthesis_fillers(&self) -> Vec<JsFiller> {
        self.0.close_parenthesis_fillers.ref_to()
    }

    #[wasm_bindgen(getter = closeParenthesisPosition)]
    pub fn close_parenthesis_position(&self) -> JsSubstringPosition {
        self.0.close_parenthesis_position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn expression(&self) -> JsValue {
        self.0.expression.as_ref().ref_to()
    }

    #[wasm_bindgen(getter = openParenthesisFillers)]
    pub fn open_parenthesis_fillers(&self) -> Vec<JsFiller> {
        self.0.open_parenthesis_fillers.ref_to()
    }

    #[wasm_bindgen(getter = openParenthesisPosition)]
    pub fn open_parenthesis_position(&self) -> JsSubstringPosition {
        self.0.open_parenthesis_position.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsGrouping) -> bool {
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

impl From<JsGrouping> for Grouping<OwnedString> {
    fn from(value: JsGrouping) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, Grouping<OwnedString>> for JsValue {
    fn from_ref(value: &'rust Grouping<OwnedString>) -> Self {
        JsGrouping(value.clone()).into()
    }
}

impl GetJsName for JsGrouping {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsGrouping {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Grouping<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {}, {})",
            JsGrouping::JS_NAME,
            self.expression.try_to_js_string()?,
            self.open_parenthesis_position.try_to_js_string()?,
            self.close_parenthesis_position.try_to_js_string()?,
            self.open_parenthesis_fillers.try_to_js_string()?,
            self.close_parenthesis_fillers.try_to_js_string()?
        ))
    }
}
