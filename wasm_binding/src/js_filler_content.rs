use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::FillerContent;

use super::js_filler_kind::JsFillerKind;
use super::traits::{FromRef, GetJsName, TryRefTo, TryToJsString, TryToJson};
use super::types::{JsOptional, OwnedString};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "FillerContent")]
pub struct JsFillerContent(FillerContent<OwnedString>);

impl JsFillerContent {
    const JS_NAME: &'static str = "FillerContent";
}

#[wasm_bindgen(js_class = FillerContent)]
impl JsFillerContent {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsFillerContent, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: JsFillerKind,
        state: &JsValue,
    ) -> Result<JsFillerContent, JsValue> {
        match (kind, JsOptional::<String>::try_from_js_value_ref(state)?) {
            (JsFillerKind::COMMENT_BLOCK, JsOptional::Value(state)) => {
                Ok(Self(FillerContent::CommentBlock(state.into())))
            }
            (JsFillerKind::COMMENT_LINE, JsOptional::Value(state)) => {
                Ok(Self(FillerContent::CommentLine(state.as_str().into())))
            }
            (
                JsFillerKind::NEWLINE,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(FillerContent::Newline)),
            (JsFillerKind::WHITESPACE, JsOptional::Value(state)) => {
                Ok(Self(FillerContent::Whitespace(state.as_str().into())))
            }
            (kind, state) => Err(JsValue::from_str(&format!(
                "Invalid arguments for {} with {} kind: {}, but got {}.",
                JsFillerContent::JS_NAME,
                kind.try_to_js_string()?,
                match state {
                    JsOptional::Value(_) => "does not need a string",
                    JsOptional::Null | JsOptional::Undefined =>
                        "needs a string",
                },
                state.try_to_js_string()?
            ))),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> JsFillerKind {
        JsFillerKind::from(&self.0)
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> JsValue {
        match &self.0 {
            FillerContent::Newline => JsValue::NULL,
            FillerContent::CommentBlock(_)
            | FillerContent::CommentLine(_)
            | FillerContent::Whitespace(_) => self.0.to_string().into(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn string(&self) -> String {
        self.0.to_string()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsFillerContent) -> bool {
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

impl<'rust> FromRef<'rust, FillerContent<OwnedString>> for JsFillerContent {
    fn from_ref(value: &'rust FillerContent<OwnedString>) -> JsFillerContent {
        Self(value.clone())
    }
}

impl<'js> FromRef<'js, JsFillerContent> for FillerContent<OwnedString> {
    fn from_ref(value: &'js JsFillerContent) -> Self {
        value.0.clone()
    }
}

impl GetJsName for JsFillerContent {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsFillerContent {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for FillerContent<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(match self {
            FillerContent::Newline => {
                format!(
                    "new {}({})",
                    JsFillerContent::JS_NAME,
                    JsFillerKind::from(self).try_to_js_string()?
                )
            }
            FillerContent::CommentBlock(_)
            | FillerContent::CommentLine(_)
            | FillerContent::Whitespace(_) => format!(
                "new {}({}, {})",
                JsFillerContent::JS_NAME,
                JsFillerKind::from(self).try_to_js_string()?,
                self.to_string().try_to_js_string()?
            ),
        })
    }
}
