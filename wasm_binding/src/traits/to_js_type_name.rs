use wasm_bindgen::JsValue;

use super::get_js_name::GetJsName;

pub(crate) trait ToJsTypeName {
    fn to_js_type_name(&self) -> String;
}

impl<T: ToJsTypeName> ToJsTypeName for &T {
    fn to_js_type_name(&self) -> String {
        T::to_js_type_name(self)
    }
}

impl ToJsTypeName for JsValue {
    fn to_js_type_name(&self) -> String {
        match js_sys::Reflect::get_prototype_of(self) {
            Ok(prototype) => prototype.constructor().name().into(),
            Err(_) => self
                .js_typeof()
                .as_string()
                .unwrap_or_else(|| "unknown type".into()),
        }
    }
}

impl ToJsTypeName for js_sys::Array {
    fn to_js_type_name(&self) -> String {
        self.constructor().name().into()
    }
}

impl ToJsTypeName for js_sys::JsString {
    fn to_js_type_name(&self) -> String {
        self.js_typeof()
            .as_string()
            .unwrap_or_else(|| "unknown type".into())
    }
}

impl ToJsTypeName for String {
    fn to_js_type_name(&self) -> String {
        js_sys::JsString::get_js_name()
    }
}
