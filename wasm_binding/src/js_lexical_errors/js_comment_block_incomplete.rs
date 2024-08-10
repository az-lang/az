use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::CommentBlockIncomplete;

use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::TokenOwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "CommentBlockIncomplete")]
pub struct JsCommentBlockIncomplete(CommentBlockIncomplete<TokenOwnedString>);

impl JsCommentBlockIncomplete {
    const JS_NAME: &'static str = "CommentBlockIncomplete";
}

#[wasm_bindgen(js_class = CommentBlockIncomplete)]
impl JsCommentBlockIncomplete {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsCommentBlockIncomplete, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(position: JsSubstringPosition, string: String) -> Self {
        Self(CommentBlockIncomplete {
            position: position.into(),
            string: string.into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsSubstringPosition {
        self.0.position.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn strings(&self) -> String {
        self.0.string.as_ref().into()
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

impl From<JsCommentBlockIncomplete>
    for CommentBlockIncomplete<TokenOwnedString>
{
    fn from(value: JsCommentBlockIncomplete) -> Self {
        value.0
    }
}

impl From<CommentBlockIncomplete<TokenOwnedString>>
    for JsCommentBlockIncomplete
{
    fn from(value: CommentBlockIncomplete<TokenOwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsCommentBlockIncomplete {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsCommentBlockIncomplete {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for CommentBlockIncomplete<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {})",
            JsCommentBlockIncomplete::JS_NAME,
            self.position.try_to_js_string()?,
            self.string.try_to_js_string()?,
        ))
    }
}
