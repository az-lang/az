use wasm_bindgen::JsValue;

use crate::utils::js_value_to_js_string;

pub(crate) trait TryToJsString {
    type Error;

    fn try_to_js_string(&self) -> Result<String, Self::Error>;
}

impl TryToJsString for String {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.as_str().try_to_js_string()
    }
}

impl TryToJsString for char {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!("'{}'", self))
    }
}

impl TryToJsString for str {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!("{:?}", self))
    }
}

impl<T: ?Sized> TryToJsString for Box<T>
where
    T: TryToJsString,
{
    type Error = <T as TryToJsString>::Error;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.as_ref().try_to_js_string()
    }
}

impl<T> TryToJsString for Option<T>
where
    T: TryToJsString,
{
    type Error = <T as TryToJsString>::Error;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        match self {
            Some(value) => value.try_to_js_string(),
            None => Ok(js_value_to_js_string(&JsValue::NULL).into()),
        }
    }
}

impl<T: TryToJsString> TryToJsString for Vec<T> {
    type Error = <T as TryToJsString>::Error;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        let mut result = String::from("[");
        result.push_str(
            &self
                .iter()
                .map(|item| item.try_to_js_string())
                .collect::<Result<Vec<_>, _>>()?
                .join(", "),
        );
        result.push(']');
        Ok(result)
    }
}

impl TryToJsString for usize {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(self.to_string())
    }
}
