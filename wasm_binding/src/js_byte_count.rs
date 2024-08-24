use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::ByteCount;

use super::traits::{FromRef, GetJsName, TryRefTo, TryToJsString, TryToJson};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "ByteCount")]
pub struct JsByteCount(ByteCount);

impl JsByteCount {
    const JS_NAME: &'static str = "ByteCount";
}

#[wasm_bindgen(js_class = ByteCount)]
impl JsByteCount {
    #[wasm_bindgen(getter = MAX)]
    pub fn max() -> Self {
        Self::new(usize::MAX)
    }

    #[wasm_bindgen(getter = MIN)]
    pub fn min() -> Self {
        Self::new(usize::MIN)
    }

    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsByteCount, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(value: usize) -> Self {
        Self(value.into())
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsByteCount) -> bool {
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

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> usize {
        self.0.into()
    }
}

impl<'rust> FromRef<'rust, ByteCount> for JsByteCount {
    fn from_ref(value: &'rust ByteCount) -> Self {
        Self(*value)
    }
}

impl<'js> FromRef<'js, JsByteCount> for ByteCount {
    fn from_ref(value: &'js JsByteCount) -> Self {
        value.0
    }
}

impl GetJsName for JsByteCount {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for ByteCount {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({})",
            JsByteCount::JS_NAME,
            usize::from(*self)
        ))
    }
}

impl TryToJsString for JsByteCount {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}
