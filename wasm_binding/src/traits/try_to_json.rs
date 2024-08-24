use wasm_bindgen::{JsError, JsValue};

pub(crate) trait TryToJson<'rust> {
    type Error;

    fn try_to_json(&'rust self) -> Result<JsValue, Self::Error>;
}

impl<'rust, T: serde::ser::Serialize> TryToJson<'rust> for T {
    type Error = JsValue;

    fn try_to_json(&'rust self) -> Result<JsValue, Self::Error> {
        let json_string = serde_json::to_string(self)
            .map_err(|error| JsValue::from(JsError::from(error)))?;
        js_sys::JSON::parse(&json_string)
    }
}
