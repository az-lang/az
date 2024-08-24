use super::to_js_type_name::ToJsTypeName;

pub(crate) trait GetJsName {
    fn get_js_name() -> String;
}

impl<T: GetJsName> GetJsName for &T {
    fn get_js_name() -> String {
        T::get_js_name()
    }
}

impl GetJsName for js_sys::Array {
    fn get_js_name() -> String {
        js_sys::Array::new().to_js_type_name()
    }
}

impl GetJsName for js_sys::JsString {
    fn get_js_name() -> String {
        js_sys::JsString::from("").to_js_type_name()
    }
}

impl GetJsName for String {
    fn get_js_name() -> String {
        js_sys::JsString::get_js_name()
    }
}
