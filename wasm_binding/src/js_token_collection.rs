use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsError, JsValue};

use az::tokenization::TokenCollection;

use super::js_lexical_errors::OwnedLexicalErrorWrapper;
use super::js_token::JsToken;
use super::traits::{
    FromRef, GetJsName, RefTo, TryFromRef, TryRefTo, TryToJsString, TryToJson,
};
use super::types::TokenOwnedString;
use super::validation::validate_positions;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "TokenCollection")]
pub struct JsTokenCollection(TokenCollection<TokenOwnedString>);

impl JsTokenCollection {
    const JS_NAME: &'static str = "TokenCollection";
}

#[wasm_bindgen(js_class = TokenCollection)]
impl JsTokenCollection {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: JsValue) -> Result<JsTokenCollection, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(tokens: &js_sys::Array) -> Result<JsTokenCollection, JsValue> {
        Ok(Self(TokenCollection::new(tokens.try_ref_to()?)))
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(value: String) -> Result<JsTokenCollection, JsError> {
        TokenCollection::<TokenOwnedString>::try_from(value.as_ref())
            .map(Self)
            .map_err(|error| OwnedLexicalErrorWrapper::from(error).into())
    }

    #[wasm_bindgen(getter)]
    pub fn string(&self) -> String {
        self.0.to_string()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsTokenCollection) -> bool {
        self == other
    }

    #[wasm_bindgen(js_name = "toArray")]
    pub fn to_array(&self) -> js_sys::Array {
        self.0.ref_to()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        self.try_to_json()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> Result<String, JsValue> {
        self.try_to_js_string()
    }

    #[wasm_bindgen(js_name = "validatePositions")]
    pub fn validate_positions(&self) -> Result<(), JsValue> {
        validate_positions(&self.0)
    }
}

impl From<JsTokenCollection> for TokenCollection<TokenOwnedString> {
    fn from(value: JsTokenCollection) -> Self {
        value.0
    }
}

impl From<TokenCollection<TokenOwnedString>> for JsTokenCollection {
    fn from(value: TokenCollection<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl<'rust> FromRef<'rust, TokenCollection<TokenOwnedString>>
    for JsTokenCollection
{
    fn from_ref(value: &'rust TokenCollection<TokenOwnedString>) -> Self {
        Self(value.clone())
    }
}

impl<'js> FromRef<'js, JsTokenCollection>
    for TokenCollection<TokenOwnedString>
{
    fn from_ref(value: &'js JsTokenCollection) -> Self {
        value.0.clone()
    }
}

impl<'rust> FromRef<'rust, TokenCollection<TokenOwnedString>>
    for js_sys::Array
{
    fn from_ref(value: &'rust TokenCollection<TokenOwnedString>) -> Self {
        value
            .clone()
            .into_iter()
            .map(|value| JsValue::from(JsToken::from(value)))
            .collect()
    }
}

impl GetJsName for JsTokenCollection {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl<'js> TryFromRef<'js, js_sys::Array>
    for TokenCollection<TokenOwnedString>
{
    type Error = JsValue;

    fn try_from_ref(value: &'js js_sys::Array) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value
                .iter()
                .map(|value| JsToken::try_from_ref(&value).map(Into::into))
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl TryToJsString for JsTokenCollection {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for TokenCollection<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({})",
            JsTokenCollection::JS_NAME,
            format_args!(
                "[{}]",
                self.iter()
                    .map(TryToJsString::try_to_js_string)
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ")
            )
        ))
    }
}
