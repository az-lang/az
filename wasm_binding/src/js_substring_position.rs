use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::SubstringPosition;

use super::js_character_position::JsCharacterPosition;
use super::traits::{
    FromRef, GetJsName, RefTo, TryFromRef, TryRefTo, TryToJsString, TryToJson,
};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "SubstringPosition")]
pub struct JsSubstringPosition(SubstringPosition);

impl JsSubstringPosition {
    const JS_NAME: &'static str = "SubstringPosition";
}

#[wasm_bindgen(js_class = SubstringPosition)]
impl JsSubstringPosition {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsSubstringPosition, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        start: &JsCharacterPosition,
        end: &JsCharacterPosition,
    ) -> Self {
        Self(SubstringPosition {
            start: start.ref_to(),
            end: end.ref_to(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> JsCharacterPosition {
        self.0.end.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> JsCharacterPosition {
        self.0.start.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsSubstringPosition) -> bool {
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

impl From<JsSubstringPosition> for SubstringPosition {
    fn from(value: JsSubstringPosition) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, SubstringPosition> for JsSubstringPosition {
    fn from_ref(value: &'rust SubstringPosition) -> Self {
        Self(value.clone())
    }
}

impl<'rust> FromRef<'rust, SubstringPosition> for JsValue {
    fn from_ref(value: &'rust SubstringPosition) -> Self {
        JsSubstringPosition::from_ref(value).into()
    }
}

impl<'js> FromRef<'js, JsSubstringPosition> for SubstringPosition {
    fn from_ref(value: &'js JsSubstringPosition) -> Self {
        value.0.clone()
    }
}

impl<'rust> FromRef<'rust, Vec<SubstringPosition>>
    for Vec<JsSubstringPosition>
{
    fn from_ref(value: &'rust Vec<SubstringPosition>) -> Self {
        value.iter().map(RefTo::ref_to).collect()
    }
}

impl GetJsName for JsSubstringPosition {
    fn get_js_name() -> String {
        Self::JS_NAME.to_string()
    }
}

impl<'js> TryFromRef<'js, js_sys::Array> for Vec<SubstringPosition> {
    type Error = JsValue;

    fn try_from_ref(value: &'js js_sys::Array) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|value| {
                JsSubstringPosition::try_from_ref(&value).map(Into::into)
            })
            .collect()
    }
}

impl TryToJsString for JsSubstringPosition {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for SubstringPosition {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {})",
            JsSubstringPosition::JS_NAME,
            self.start.try_to_js_string()?,
            self.end.try_to_js_string()?
        ))
    }
}
