use wasm_bindgen::JsValue;

use az::parsing::Statement;

use crate::traits::{FromRef, RefTo, TryFromRef, TryToJsString};
use crate::types::OwnedString;

use super::js_expression_statement::JsExpressionStatement;

impl<'rust> FromRef<'rust, Statement<OwnedString>> for JsValue {
    fn from_ref(value: &'rust Statement<OwnedString>) -> Self {
        match value {
            Statement::Expression(value) => value.ref_to(),
        }
    }
}

impl<'rust> FromRef<'rust, Vec<Statement<OwnedString>>> for Vec<JsValue> {
    fn from_ref(value: &'rust Vec<Statement<OwnedString>>) -> Self {
        value.iter().map(FromRef::from_ref).collect()
    }
}

impl<'js> TryFromRef<'js, JsValue> for Statement<OwnedString> {
    type Error = JsValue;

    fn try_from_ref(value: &'js JsValue) -> Result<Self, Self::Error> {
        JsExpressionStatement::try_from_ref(value)
            .map(|value| Statement::Expression(value.into()))
    }
}

impl<'js> TryFromRef<'js, js_sys::Array> for Vec<Statement<OwnedString>> {
    type Error = JsValue;

    fn try_from_ref(value: &'js js_sys::Array) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|value| Statement::try_from_ref(&value))
            .collect()
    }
}

impl TryToJsString for Statement<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        match self {
            Statement::Expression(value) => value.try_to_js_string(),
        }
    }
}
