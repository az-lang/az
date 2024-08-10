use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::Token;

use super::js_substring_position::JsSubstringPosition;
use super::js_token_content::JsTokenContent;
use super::traits::{
    FromRef, GetJsName, RefTo, TryFromRef, TryRefTo, TryToJsString, TryToJson,
};
use super::types::TokenOwnedString;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Token")]
pub struct JsToken(Token<TokenOwnedString>);

impl JsToken {
    const JS_NAME: &'static str = "Token";
}

#[wasm_bindgen(js_class = Token)]
impl JsToken {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: JsValue) -> Result<JsToken, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        content: &JsTokenContent,
        position: &JsSubstringPosition,
    ) -> Self {
        Self(Token {
            content: content.ref_to(),
            position: position.ref_to(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> JsTokenContent {
        self.0.content.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsSubstringPosition {
        self.0.position.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsToken) -> bool {
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
}

impl From<JsToken> for Token<TokenOwnedString> {
    fn from(value: JsToken) -> Self {
        value.0
    }
}

impl From<Token<TokenOwnedString>> for JsToken {
    fn from(value: Token<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl<'rust> FromRef<'rust, Token<TokenOwnedString>> for JsToken {
    fn from_ref(value: &'rust Token<TokenOwnedString>) -> Self {
        Self(value.clone())
    }
}

impl<'js> FromRef<'js, JsToken> for Token<TokenOwnedString> {
    fn from_ref(value: &'js JsToken) -> Self {
        value.0.clone()
    }
}

impl GetJsName for JsToken {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl<'js> TryFromRef<'js, js_sys::Array> for Vec<Token<TokenOwnedString>> {
    type Error = JsValue;

    fn try_from_ref(value: &'js js_sys::Array) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|value| JsToken::try_from_ref(&value).map(Into::into))
            .collect()
    }
}

impl TryToJsString for JsToken {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Token<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {})",
            JsToken::JS_NAME,
            self.content.try_to_js_string()?,
            self.position.try_to_js_string()?
        ))
    }
}
