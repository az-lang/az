use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::FillerContent;

use super::traits::TryToJsString;
use super::types::OwnedString;

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "FillerKind")]
pub enum JsFillerKind {
    COMMENT_BLOCK,
    COMMENT_LINE,
    NEWLINE,
    WHITESPACE,
}

impl From<&FillerContent<OwnedString>> for JsFillerKind {
    fn from(value: &FillerContent<OwnedString>) -> Self {
        match value {
            FillerContent::CommentBlock(_) => Self::COMMENT_BLOCK,
            FillerContent::CommentLine(_) => Self::COMMENT_LINE,
            FillerContent::Newline => Self::NEWLINE,
            FillerContent::Whitespace(_) => Self::WHITESPACE,
        }
    }
}

impl TryToJsString for JsFillerKind {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "FillerKind.{}",
            match self {
                Self::COMMENT_BLOCK => "COMMENT_BLOCK",
                Self::COMMENT_LINE => "COMMENT_LINE",
                Self::NEWLINE => "NEWLINE",
                Self::WHITESPACE => "WHITESPACE",
            }
        ))
    }
}
