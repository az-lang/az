use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::MissingSemicolon;

use crate::js_token::JsToken;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "MissingSemicolon")]
pub struct JsMissingSemicolon(MissingSemicolon<TokenOwnedString>);

impl JsMissingSemicolon {
    const JS_NAME: &'static str = "MissingSemicolon";
}

#[wasm_bindgen(js_class = MissingSemicolon)]
impl JsMissingSemicolon {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsMissingSemicolon, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(token: &JsToken) -> Self {
        Self(MissingSemicolon {
            token: token.ref_to(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn token(&self) -> JsToken {
        self.0.token.ref_to()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        self.try_to_json()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> Result<String, JsValue> {
        self.try_to_js_string()
    }
}

impl From<JsMissingSemicolon> for MissingSemicolon<TokenOwnedString> {
    fn from(value: JsMissingSemicolon) -> Self {
        value.0
    }
}

impl From<MissingSemicolon<TokenOwnedString>> for JsMissingSemicolon {
    fn from(value: MissingSemicolon<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsMissingSemicolon {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsMissingSemicolon {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for MissingSemicolon<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({})",
            JsMissingSemicolon::JS_NAME,
            self.token.try_to_js_string()?
        ))
    }
}
