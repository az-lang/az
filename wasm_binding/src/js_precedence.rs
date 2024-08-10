use wasm_bindgen::prelude::wasm_bindgen;

use az::parsing::Precedence;

#[derive(Clone, PartialEq, PartialOrd)]
#[wasm_bindgen(js_name = "Precedence")]
pub struct JsPrecedence(Precedence);

#[wasm_bindgen(js_class = Precedence)]
impl JsPrecedence {
    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsPrecedence) -> bool {
        self == other
    }

    #[wasm_bindgen(js_name = "greaterThan")]
    pub fn greater_than(&self, other: &JsPrecedence) -> bool {
        self > other
    }

    #[wasm_bindgen(js_name = "greaterThanOrEqualTo")]
    pub fn greater_than_or_equal_to(&self, other: &JsPrecedence) -> bool {
        self >= other
    }

    #[wasm_bindgen(js_name = "lessThan")]
    pub fn less_than(&self, other: &JsPrecedence) -> bool {
        self < other
    }

    #[wasm_bindgen(js_name = "lessThanOrEqualTo")]
    pub fn less_than_or_equal_to(&self, other: &JsPrecedence) -> bool {
        self <= other
    }
}

impl From<Precedence> for JsPrecedence {
    fn from(value: Precedence) -> Self {
        Self(value)
    }
}

impl From<JsPrecedence> for Precedence {
    fn from(value: JsPrecedence) -> Self {
        value.0
    }
}
