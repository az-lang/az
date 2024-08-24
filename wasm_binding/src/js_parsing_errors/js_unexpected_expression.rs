use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::UnexpectedExpression;

use crate::traits::{GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson};
use crate::types::OwnedString;

#[derive(serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "UnexpectedExpression")]
pub struct JsUnexpectedExpression(UnexpectedExpression<OwnedString>);

impl JsUnexpectedExpression {
    const JS_NAME: &'static str = "UnexpectedExpression";
}

#[wasm_bindgen(js_class = UnexpectedExpression)]
impl JsUnexpectedExpression {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(
        value: &JsValue,
    ) -> Result<JsUnexpectedExpression, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        expression: &JsValue,
    ) -> Result<JsUnexpectedExpression, JsValue> {
        Ok(Self(UnexpectedExpression {
            expression: expression.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn expression(&self) -> JsValue {
        self.0.expression.ref_to()
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

impl From<JsUnexpectedExpression> for UnexpectedExpression<OwnedString> {
    fn from(value: JsUnexpectedExpression) -> Self {
        value.0
    }
}

impl From<UnexpectedExpression<OwnedString>> for JsUnexpectedExpression {
    fn from(value: UnexpectedExpression<OwnedString>) -> Self {
        Self(value)
    }
}

impl GetJsName for JsUnexpectedExpression {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsUnexpectedExpression {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for UnexpectedExpression<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({})",
            JsUnexpectedExpression::JS_NAME,
            self.expression.try_to_js_string()?
        ))
    }
}
