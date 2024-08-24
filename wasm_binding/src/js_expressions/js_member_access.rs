use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::parsing::MemberAccess;

use crate::js_filler::JsFiller;
use crate::js_substring_position::JsSubstringPosition;
use crate::traits::{
    FromRef, GetJsName, RefTo, TryRefTo, TryToJsString, TryToJson,
};
use crate::types::OwnedString;
use crate::validation::{validate_contents, validate_positions};

use super::js_identifier::JsIdentifier;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "MemberAccess")]
pub struct JsMemberAccess(MemberAccess<OwnedString>);

impl JsMemberAccess {
    const JS_NAME: &'static str = "MemberAccess";
}

#[wasm_bindgen(js_class = MemberAccess)]
impl JsMemberAccess {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: &JsValue) -> Result<JsMemberAccess, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        object: &JsValue,
        member: &JsIdentifier,
        #[allow(non_snake_case)] operatorPosition: &JsSubstringPosition,
        #[allow(non_snake_case)] operatorFillers: &js_sys::Array,
    ) -> Result<JsMemberAccess, JsValue> {
        Ok(JsMemberAccess(MemberAccess {
            object: Box::new(object.try_ref_to()?),
            member: member.ref_to(),
            operator_position: operatorPosition.ref_to(),
            operator_fillers: operatorFillers.try_ref_to()?,
        }))
    }

    #[wasm_bindgen(getter)]
    pub fn member(&self) -> JsIdentifier {
        self.0.member.ref_to()
    }

    #[wasm_bindgen(getter)]
    pub fn object(&self) -> JsValue {
        self.0.object.as_ref().ref_to()
    }

    #[wasm_bindgen(getter = operatorFillers)]
    pub fn operator_fillers(&self) -> Vec<JsFiller> {
        self.0.operator_fillers.ref_to()
    }

    #[wasm_bindgen(getter = operatorPosition)]
    pub fn operator_position(&self) -> JsSubstringPosition {
        self.0.operator_position.ref_to()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsMemberAccess) -> bool {
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

    #[wasm_bindgen(js_name = "validateContents")]
    pub fn validate_contents(&self) -> Result<(), JsValue> {
        validate_contents(&self.0)
    }

    #[wasm_bindgen(js_name = "validatePositions")]
    pub fn validate_positions(&self) -> Result<(), JsValue> {
        validate_positions(&self.0)
    }
}

impl From<JsMemberAccess> for MemberAccess<OwnedString> {
    fn from(value: JsMemberAccess) -> Self {
        value.0
    }
}

impl<'rust> FromRef<'rust, MemberAccess<OwnedString>> for JsValue {
    fn from_ref(value: &'rust MemberAccess<OwnedString>) -> Self {
        JsMemberAccess(value.clone()).into()
    }
}

impl GetJsName for JsMemberAccess {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsMemberAccess {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for MemberAccess<OwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(format!(
            "new {}({}, {}, {}, {})",
            JsMemberAccess::JS_NAME,
            self.object.try_to_js_string()?,
            self.member.try_to_js_string()?,
            self.operator_position.try_to_js_string()?,
            self.operator_fillers.try_to_js_string()?
        ))
    }
}
