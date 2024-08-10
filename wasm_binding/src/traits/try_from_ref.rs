use wasm_bindgen::{JsError, JsValue};

use crate::utils::to_invalid_js_value_message;

use super::get_js_name::GetJsName;

pub(crate) trait TryFromRef<'a, T>: Sized {
    type Error;

    fn try_from_ref(value: &'a T) -> Result<Self, Self::Error>;
}

impl<'js, T: GetJsName + for<'a> serde::de::Deserialize<'a>>
    TryFromRef<'js, JsValue> for T
{
    type Error = JsValue;

    fn try_from_ref(value: &'js JsValue) -> Result<Self, Self::Error> {
        if value.is_undefined() {
            Err(JsValue::from(&to_invalid_js_value_message::<T>(value)))
        } else {
            serde_json::from_str(
                &js_sys::JSON::stringify(value).map(String::from)?,
            )
            .map_err(|error| JsValue::from(JsError::from(error)))
        }
    }
}
