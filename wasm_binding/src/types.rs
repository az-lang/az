use js_sys::JsString;
use wasm_bindgen::JsValue;

use super::traits::{GetJsName, ToJsTypeName, TryFromRef, TryToJsString};
use super::utils::js_value_to_js_string;

pub(crate) type OwnedString = Box<str>;
pub(crate) type TokenOwnedString = Box<str>;

pub(crate) enum JsOptional<T> {
    Null,
    Undefined,
    Value(T),
}

impl<T: GetJsName> GetJsName for JsOptional<T> {
    fn get_js_name() -> String {
        format!(
            "{} | {} | {}",
            T::get_js_name(),
            to_js_null_js_string(),
            to_js_undefined_js_string()
        )
    }
}

impl<T: ToJsTypeName> ToJsTypeName for JsOptional<T> {
    fn to_js_type_name(&self) -> String {
        match self {
            JsOptional::Value(value) => value.to_js_type_name(),
            JsOptional::Null => to_js_null_js_string().into(),
            JsOptional::Undefined => to_js_undefined_js_string().into(),
        }
    }
}

impl<'js, T: TryFromRef<'js, JsValue>> JsOptional<T> {
    pub(crate) fn try_from_js_value_ref(
        value: &'js JsValue,
    ) -> Result<Self, <T as TryFromRef<'js, JsValue>>::Error> {
        if value.is_null() {
            Ok(JsOptional::Null)
        } else if value.is_undefined() {
            Ok(JsOptional::Undefined)
        } else {
            T::try_from_ref(value).map(JsOptional::Value)
        }
    }
}

impl<T: TryToJsString> TryToJsString for JsOptional<T> {
    type Error = <T as TryToJsString>::Error;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        match self {
            JsOptional::Null => Ok(to_js_null_js_string().into()),
            JsOptional::Undefined => Ok(to_js_undefined_js_string().into()),
            JsOptional::Value(value) => value.try_to_js_string(),
        }
    }
}

fn to_js_null_js_string() -> JsString {
    js_value_to_js_string(&JsValue::NULL)
}

fn to_js_undefined_js_string() -> JsString {
    js_value_to_js_string(&JsValue::UNDEFINED)
}
