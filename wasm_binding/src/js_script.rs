use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsError, JsValue};

use az::parsing::Script;
use az::tokenization::{TokenCollection, Tokenize};

use super::js_filler::JsFiller;
use super::js_parsing_errors::OwnedParsingErrorWrapper;
use super::js_token_collection::JsTokenCollection;
use super::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use super::types::OwnedString;
use super::validation::{validate_contents, validate_positions};

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Script")]
pub struct JsScript(Script<OwnedString>);

impl JsScript {
    const JS_NAME: &'static str = "Script";
}

#[wasm_bindgen(js_class = Script)]
impl JsScript {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsScript, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(js_name = "fromTokens")]
    pub fn from_tokens(
        tokens: &JsTokenCollection,
    ) -> Result<JsScript, JsValue> {
        Script::try_from(TokenCollection::from_ref(tokens))
            .map(Self)
            .map_err(|error| {
                JsError::from(OwnedParsingErrorWrapper::from(error)).into()
            })
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        statements: &js_sys::Array,
        fillers: &js_sys::Array,
    ) -> Result<JsScript, JsValue> {
        Ok(JsScript(Script {
            statements: statements.try_ref_to()?,
            fillers: fillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn fillers(&self) -> Vec<JsFiller> {
        self.0.fillers.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn statements(&self) -> Vec<JsValue> {
        self.0.statements.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsScript) -> bool {
        self == other
    }

    pub fn format(&mut self) {
        self.0.format()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        self.try_to_json()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> Result<String, JsValue> {
        self.try_to_js_string()
    }

    pub fn tokenize(&self) -> JsTokenCollection {
        self.0.clone().tokenize().into()
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

impl GetJsName for JsScript {
    fn get_js_name() -> String {
        Self::JS_NAME.to_string()
    }
}

impl TryToJsString for JsScript {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Script<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {})",
            JsScript::JS_NAME,
            self.statements.try_to_js_string()?,
            self.fillers.try_to_js_string()?,
        ))
    }
}
