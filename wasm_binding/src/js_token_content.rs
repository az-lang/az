use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use az::tokenization::{NumericLiteralType, TokenContent};

use super::js_token_kind::JsTokenKind;
use super::traits::{FromRef, GetJsName, TryRefTo, TryToJsString, TryToJson};
use super::types::{JsOptional, TokenOwnedString};

#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[wasm_bindgen(js_name = "TokenContent")]
pub struct JsTokenContent(TokenContent<TokenOwnedString>);

impl JsTokenContent {
    const JS_NAME: &'static str = "TokenContent";
}

#[wasm_bindgen(js_class = TokenContent)]
impl JsTokenContent {
    #[wasm_bindgen(js_name = "fromJSON")]
    pub fn from_json(value: JsValue) -> Result<JsTokenContent, JsValue> {
        value.try_ref_to()
    }

    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: JsTokenKind,
        state: &JsValue,
    ) -> Result<JsTokenContent, JsValue> {
        match (kind, JsOptional::<String>::try_from_js_value_ref(state)?) {
            (JsTokenKind::ARROW, JsOptional::Null | JsOptional::Undefined) => {
                Ok(Self(TokenContent::Arrow))
            }
            (
                JsTokenKind::ASSIGNMENT,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::Assignment)),
            (
                JsTokenKind::ASTERISK,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::Asterisk)),
            (
                JsTokenKind::CLOSE_BRACE,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::CloseBrace)),
            (
                JsTokenKind::CLOSE_PARENTHESIS,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::CloseParenthesis)),
            (JsTokenKind::COLON, JsOptional::Null | JsOptional::Undefined) => {
                Ok(Self(TokenContent::Colon))
            }
            (JsTokenKind::COMMA, JsOptional::Null | JsOptional::Undefined) => {
                Ok(Self(TokenContent::Comma))
            }
            (JsTokenKind::COMMENT_BLOCK, JsOptional::Value(state)) => {
                Ok(Self(TokenContent::CommentBlock(state.into())))
            }
            (JsTokenKind::COMMENT_LINE, JsOptional::Value(state)) => {
                Ok(Self(TokenContent::CommentLine(state.into())))
            }
            (JsTokenKind::DOT, JsOptional::Null | JsOptional::Undefined) => {
                Ok(Self(TokenContent::Dot))
            }
            (
                JsTokenKind::EQUAL_TO,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::EqualTo)),
            (
                kind @ (JsTokenKind::F32
                | JsTokenKind::F64
                | JsTokenKind::I8
                | JsTokenKind::I16
                | JsTokenKind::I32
                | JsTokenKind::I64
                | JsTokenKind::ISIZE
                | JsTokenKind::U8
                | JsTokenKind::U16
                | JsTokenKind::U32
                | JsTokenKind::U64
                | JsTokenKind::USIZE),
                JsOptional::Value(state),
            ) => {
                let type_ = match kind {
                    JsTokenKind::F32 => NumericLiteralType::F32,
                    JsTokenKind::F64 => NumericLiteralType::F64,
                    JsTokenKind::I8 => NumericLiteralType::I8,
                    JsTokenKind::I16 => NumericLiteralType::I16,
                    JsTokenKind::I32 => NumericLiteralType::I32,
                    JsTokenKind::I64 => NumericLiteralType::I64,
                    JsTokenKind::ISIZE => NumericLiteralType::ISize,
                    JsTokenKind::U8 => NumericLiteralType::U8,
                    JsTokenKind::U16 => NumericLiteralType::U16,
                    JsTokenKind::U32 => NumericLiteralType::U32,
                    JsTokenKind::U64 => NumericLiteralType::U64,
                    JsTokenKind::USIZE => NumericLiteralType::USize,
                    _ => unreachable!(),
                };
                Ok(Self(TokenContent::NumericLiteral {
                    value: state.into(),
                    type_,
                }))
            }
            (
                JsTokenKind::GREATER_THAN,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::GreaterThan)),
            (
                JsTokenKind::GREATER_THAN_OR_EQUAL_TO,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::GreaterThanOrEqualTo)),
            (JsTokenKind::IDENTIFIER, JsOptional::Value(state)) => {
                Ok(Self(TokenContent::Identifier(state.into())))
            }
            (
                JsTokenKind::LESS_THAN,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::LessThan)),
            (
                JsTokenKind::LESS_THAN_OR_EQUAL_TO,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::LessThanOrEqualTo)),
            (JsTokenKind::MINUS, JsOptional::Null | JsOptional::Undefined) => {
                Ok(Self(TokenContent::Minus))
            }
            (
                JsTokenKind::NEWLINE,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::Newline)),
            (
                JsTokenKind::NOT_EQUAL_TO,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::NotEqualTo)),
            (
                JsTokenKind::OPEN_BRACE,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::OpenBrace)),
            (
                JsTokenKind::OPEN_PARENTHESIS,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::OpenParenthesis)),
            (JsTokenKind::PLUS, JsOptional::Null | JsOptional::Undefined) => {
                Ok(Self(TokenContent::Plus))
            }
            (
                JsTokenKind::SEMICOLON,
                JsOptional::Null | JsOptional::Undefined,
            ) => Ok(Self(TokenContent::Semicolon)),
            (JsTokenKind::SLASH, JsOptional::Null | JsOptional::Undefined) => {
                Ok(Self(TokenContent::Slash))
            }
            (JsTokenKind::WHITESPACE, JsOptional::Value(state)) => {
                Ok(Self(TokenContent::Whitespace(state.into())))
            }
            (kind, state) => Err(JsValue::from_str(&format!(
                "Invalid arguments for {} with {} kind: {}, but got {}.",
                JsTokenContent::JS_NAME,
                kind.try_to_js_string()?,
                match state {
                    JsOptional::Value(_) => "does not need a state",
                    JsOptional::Null | JsOptional::Undefined =>
                        "needs a state",
                },
                state.try_to_js_string()?
            ))),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> JsTokenKind {
        JsTokenKind::from(&self.0)
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> JsValue {
        match &self.0 {
            TokenContent::Arrow
            | TokenContent::Assignment
            | TokenContent::Asterisk
            | TokenContent::CloseBrace
            | TokenContent::CloseParenthesis
            | TokenContent::Colon
            | TokenContent::Comma
            | TokenContent::Dot
            | TokenContent::EqualTo
            | TokenContent::GreaterThan
            | TokenContent::GreaterThanOrEqualTo
            | TokenContent::LessThan
            | TokenContent::LessThanOrEqualTo
            | TokenContent::Minus
            | TokenContent::Newline
            | TokenContent::NotEqualTo
            | TokenContent::OpenBrace
            | TokenContent::OpenParenthesis
            | TokenContent::Plus
            | TokenContent::Semicolon
            | TokenContent::Slash => JsValue::NULL,
            TokenContent::CommentBlock(_)
            | TokenContent::CommentLine(_)
            | TokenContent::Identifier(_)
            | TokenContent::Whitespace(_) => self.0.to_string().into(),
            TokenContent::NumericLiteral { value, .. } => {
                String::from(value.clone()).into()
            }
        }
    }

    #[wasm_bindgen(getter)]
    pub fn string(&self) -> String {
        self.0.to_string()
    }

    #[wasm_bindgen(js_name = "equalTo")]
    pub fn equal_to(&self, other: &JsTokenContent) -> bool {
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
}

impl<'rust> FromRef<'rust, TokenContent<TokenOwnedString>> for JsTokenContent {
    fn from_ref(value: &'rust TokenContent<TokenOwnedString>) -> Self {
        Self(value.clone())
    }
}

impl<'rust> FromRef<'rust, JsTokenContent> for TokenContent<TokenOwnedString> {
    fn from_ref(value: &'rust JsTokenContent) -> Self {
        value.0.clone()
    }
}

impl GetJsName for JsTokenContent {
    fn get_js_name() -> String {
        Self::JS_NAME.into()
    }
}

impl TryToJsString for JsTokenContent {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        self.0.try_to_js_string()
    }
}

impl TryToJsString for TokenContent<TokenOwnedString> {
    type Error = JsValue;

    fn try_to_js_string(&self) -> Result<String, Self::Error> {
        Ok(match self {
            TokenContent::Arrow
            | TokenContent::Assignment
            | TokenContent::Asterisk
            | TokenContent::CloseBrace
            | TokenContent::CloseParenthesis
            | TokenContent::Colon
            | TokenContent::Comma
            | TokenContent::Dot
            | TokenContent::EqualTo
            | TokenContent::GreaterThan
            | TokenContent::GreaterThanOrEqualTo
            | TokenContent::LessThan
            | TokenContent::LessThanOrEqualTo
            | TokenContent::Minus
            | TokenContent::Newline
            | TokenContent::NotEqualTo
            | TokenContent::OpenBrace
            | TokenContent::OpenParenthesis
            | TokenContent::Plus
            | TokenContent::Semicolon
            | TokenContent::Slash => {
                format!(
                    "new {}({})",
                    JsTokenContent::JS_NAME,
                    JsTokenKind::from(self).try_to_js_string()?
                )
            }
            TokenContent::CommentBlock(_)
            | TokenContent::CommentLine(_)
            | TokenContent::Identifier(_)
            | TokenContent::Whitespace(_) => format!(
                "new {}({}, {})",
                JsTokenContent::JS_NAME,
                JsTokenKind::from(self).try_to_js_string()?,
                self.to_string().try_to_js_string()?
            ),
            TokenContent::NumericLiteral { value, .. } => format!(
                "new {}({}, {})",
                JsTokenContent::JS_NAME,
                JsTokenKind::from(self).try_to_js_string()?,
                value.try_to_js_string()?
            ),
        })
    }
}
