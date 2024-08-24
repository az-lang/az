use crate::traits::{GetJsName, ToJsTypeName};
use wasm_bindgen::{JsCast, JsValue};

pub(crate) fn js_value_to_js_string(value: &JsValue) -> js_sys::JsString {
    if let Some(value) = value.dyn_ref::<js_sys::Object>() {
        js_sys::Object::constructor(value).name()
    } else {
        js_sys::JsString::from(value + JsValue::from(""))
    }
}

pub(crate) fn to_invalid_js_value_message<T: GetJsName>(
    value: &JsValue,
) -> String {
    format!(
        "Expected {}, but got {}.",
        T::get_js_name(),
        value.to_js_type_name()
    )
}
