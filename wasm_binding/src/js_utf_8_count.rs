use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::Utf8Count;

use super::traits::{FromRef, GetJsName, TryRefTo, TryToJsString, TryToJson};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "Utf8Count")]
pub struct JsUtf8Count(Utf8Count);

impl JsUtf8Count {
    const JS_NAME: &'static str = "Utf8Count";
}

#[wasm_bindgen(js_class = Utf8Count)]
impl JsUtf8Count {
    #[wasm_bindgen(getter = MAX)]
    pub fn max() -> Self {
        Self::new(usize::MAX)
    }

    #[wasm_bindgen(getter = MIN)]
    pub fn min() -> Self {
        Self::new(usize::MIN)
    }

    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsUtf8Count, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(value: usize) -> Self {
        Self(value.into())
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsUtf8Count) -> bool {
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

impl<'rust> FromRef<'rust, Utf8Count> for JsUtf8Count {
    fn from_ref(value: &'rust Utf8Count) -> Self {
        Self(*value)
    }
}

impl<'js> FromRef<'js, JsUtf8Count> for Utf8Count {
    fn from_ref(value: &'js JsUtf8Count) -> Self {
        value.0
    }
}

impl GetJsName for JsUtf8Count {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsUtf8Count {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for Utf8Count {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({})",
            JsUtf8Count::JS_NAME,
            usize::from(*self)
        ))
    }
}
