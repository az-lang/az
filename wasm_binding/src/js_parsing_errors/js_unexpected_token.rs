use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::UnexpectedToken;

use crate::js_token::JsToken;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "UnexpectedToken")]
pub struct JsUnexpectedToken(UnexpectedToken<TokenOwnedString>);

impl JsUnexpectedToken {
    const JS_NAME: &'static str = "UnexpectedToken";
}

#[wasm_bindgen(js_class = UnexpectedToken)]
impl JsUnexpectedToken {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsUnexpectedToken, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(token: &JsToken) -> JsUnexpectedToken {
        Self(UnexpectedToken {
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

impl From<JsUnexpectedToken> for UnexpectedToken<TokenOwnedString> {
    fn from(value: JsUnexpectedToken) -> Self {
        value.0
    }
}

impl From<UnexpectedToken<TokenOwnedString>> for JsUnexpectedToken {
    fn from(value: UnexpectedToken<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsUnexpectedToken {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsUnexpectedToken {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for UnexpectedToken<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({})",
            JsUnexpectedToken::JS_NAME,
            self.token.try_to_js_string()?
        ))
    }
}
