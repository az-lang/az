use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

use proptest::prelude::{proptest, Strategy};
use proptest::string::string_regex;

use az::tokenization::{
    NumericLiteralType, Token, TokenCollection, TokenContent,
};
use common::patterns::{
    to_floating_point_value_pattern, to_integer_value_pattern,
};
use common::strategy_factories::{
    to_comment_block_string_strategy, to_comment_line_string_strategy,
    to_identifier_string_strategy, to_whitespace_string_strategy,
};

mod common;

#[test]
fn test_empty() {
    macro_rules! assertion {
        ($string_type:ty) => {
            assert!(matches!(
                TokenCollection::<$string_type>::try_from("")
                    .map(|tokens| tokens.into_iter().collect::<Vec<_>>())
                    .as_deref(),
                Ok([])
            ));
        };
    }

    assertion!(&str);
    assertion!(Arc<str>);
    assertion!(Box<str>);
    assertion!(Cow<'_, str>);
    assertion!(Rc<str>);
    assertion!(String);
}

proptest! {
    #[test]
    fn test_doesnt_crash(text in r".*") {
        let text = text.as_str();
        drop(TokenCollection::<&str>::try_from(text));
        drop(TokenCollection::<String>::try_from(text));
        drop(TokenCollection::<Arc<str>>::try_from(text));
        drop(TokenCollection::<Box<str>>::try_from(text));
        drop(TokenCollection::<Rc<str>>::try_from(text));
    }
}

proptest! {
    #[test]
    fn test_comment_block(text in to_comment_block_string_strategy()) {
        macro_rules! assertion {
            ($string_type:ty) => {
                assert!(matches!(
                    TokenCollection::<$string_type>::try_from(text.as_str())
                        .map(|tokens| tokens.into_iter().collect::<Vec<_>>())
                        .as_deref(),
                    Ok([Token { content: TokenContent::CommentBlock(_), .. }])
                ));
            };
        }

        assertion!(&str);
        assertion!(Arc<str>);
        assertion!(Box<str>);
        assertion!(Cow<'_, str>);
        assertion!(Rc<str>);
        assertion!(String);
    }
}

proptest! {
    #[test]
    fn test_comment_line(text in to_comment_line_string_strategy()) {
        macro_rules! assertion {
            ($string_type:ty) => {
                assert!(matches!(
                    TokenCollection::<$string_type>::try_from(text.as_str())
                        .map(|tokens| tokens.into_iter().collect::<Vec<_>>())
                        .as_deref(),
                    Ok([Token { content: TokenContent::CommentLine(_), .. }])
                ));
            };
        }

        assertion!(&str);
        assertion!(Arc<str>);
        assertion!(Box<str>);
        assertion!(Cow<'_, str>);
        assertion!(Rc<str>);
        assertion!(String);
    }
}

proptest! {
    #[test]
    fn test_identifier(text in to_identifier_string_strategy()) {
        macro_rules! assertion {
            ($string_type:ty) => {
                assert!(matches!(
                    TokenCollection::<$string_type>::try_from(text.as_str())
                        .map(|tokens| tokens.into_iter().collect::<Vec<_>>())
                        .as_deref(),
                    Ok([Token { content: TokenContent::Identifier(_), .. }])
                ));
            };
        }

        assertion!(&str);
        assertion!(Arc<str>);
        assertion!(Box<str>);
        assertion!(Cow<'_, str>);
        assertion!(Rc<str>);
        assertion!(String);
    }
}

proptest! {
    #[test]
    fn test_whitespace(text in to_whitespace_string_strategy()) {
        macro_rules! assertion {
            ($string_type:ty) => {
                assert!(matches!(
                    TokenCollection::<$string_type>::try_from(text.as_str())
                        .map(|tokens| tokens.into_iter().collect::<Vec<_>>())
                        .as_deref(),
                    Ok([Token { content: TokenContent::Whitespace(_), .. }])
                ));
            };
        }
        assertion!(&str);
        assertion!(Arc<str>);
        assertion!(Box<str>);
        assertion!(Cow<'_, str>);
        assertion!(Rc<str>);
        assertion!(String);
    }
}

fn to_floating_point_literal_strategy(
    type_: NumericLiteralType,
) -> impl Strategy<Value = String> {
    debug_assert!(matches!(
        type_,
        NumericLiteralType::F32 | NumericLiteralType::F64
    ));
    string_regex(&format!(
        r"(?-u){}_{}",
        to_floating_point_value_pattern(),
        type_
    ))
    .unwrap()
}

#[test]
fn test_floating_point_literals() {
    macro_rules! run_sub_test {
        ($string_type:ty, $type_:expr) => {
            proptest!(
                |(text in to_floating_point_literal_strategy($type_),)| {
                    assert!(matches!(
                        TokenCollection::<$string_type>::try_from(text.as_str())
                            .map(|tokens| tokens.into_iter().collect::<Vec<_>>())
                            .as_deref(),
                        Ok([Token {
                            content: TokenContent::NumericLiteral { value, type_ },
                            ..
                        }])
                        if text.starts_with(AsRef::<str>::as_ref(value))
                        && text.chars().nth(value.len()).unwrap() == '_'
                        && type_.eq(&$type_)
                    ));
                }
            );
        };
        ($string_type:ty) => {
            run_sub_test!($string_type, NumericLiteralType::F32);
            run_sub_test!($string_type, NumericLiteralType::F64);
        };
    }

    run_sub_test!(&str);
    run_sub_test!(Arc<str>);
    run_sub_test!(Box<str>);
    run_sub_test!(Cow<'_, str>);
    run_sub_test!(Rc<str>);
    run_sub_test!(String);
}

fn to_integer_literal_strategy(
    type_: NumericLiteralType,
) -> impl Strategy<Value = String> {
    assert!(matches!(
        type_,
        NumericLiteralType::I8
            | NumericLiteralType::I16
            | NumericLiteralType::I32
            | NumericLiteralType::I64
            | NumericLiteralType::ISize
            | NumericLiteralType::U8
            | NumericLiteralType::U16
            | NumericLiteralType::U32
            | NumericLiteralType::U64
            | NumericLiteralType::USize
    ));
    string_regex(&format!(r"(?-u){}_{}", to_integer_value_pattern(), type_))
        .unwrap()
}

#[test]
fn test_integer_literals() {
    macro_rules! run_sub_test {
        ($string_type:ty, $type_:expr) => {
            proptest!(
                |(text in to_integer_literal_strategy($type_),)| {
                    assert!(matches!(
                        TokenCollection::<$string_type>::try_from(text.as_str())
                            .map(|tokens| tokens.into_iter().collect::<Vec<_>>())
                            .as_deref(),
                        Ok([Token {
                            content: TokenContent::NumericLiteral { value, type_ },
                            ..
                        }])
                        if text.starts_with(AsRef::<str>::as_ref(value))
                        && text.chars().nth(value.len()).unwrap() == '_'
                        && type_.eq(&$type_)
                    ));
                }
            );
        };
        ($string_type:ty) => {
            run_sub_test!($string_type, NumericLiteralType::I8);
            run_sub_test!($string_type, NumericLiteralType::I16);
            run_sub_test!($string_type, NumericLiteralType::I32);
            run_sub_test!($string_type, NumericLiteralType::I64);
            run_sub_test!($string_type, NumericLiteralType::ISize);
            run_sub_test!($string_type, NumericLiteralType::U8);
            run_sub_test!($string_type, NumericLiteralType::U16);
            run_sub_test!($string_type, NumericLiteralType::U32);
            run_sub_test!($string_type, NumericLiteralType::U64);
            run_sub_test!($string_type, NumericLiteralType::USize);
        };
    }

    run_sub_test!(&str);
    run_sub_test!(Arc<str>);
    run_sub_test!(Box<str>);
    run_sub_test!(Cow<'_, str>);
    run_sub_test!(Rc<str>);
    run_sub_test!(String);
}

#[test]
fn test_stateless_tokens() {
    macro_rules! assertion {
        ($string_type:ty) => {
            for token_content in [
                TokenContent::<$string_type>::Arrow,
                TokenContent::<$string_type>::Assignment,
                TokenContent::<$string_type>::Asterisk,
                TokenContent::<$string_type>::CloseBrace,
                TokenContent::<$string_type>::CloseParenthesis,
                TokenContent::<$string_type>::Colon,
                TokenContent::<$string_type>::Comma,
                TokenContent::<$string_type>::Dot,
                TokenContent::<$string_type>::EqualTo,
                TokenContent::<$string_type>::GreaterThan,
                TokenContent::<$string_type>::GreaterThanOrEqualTo,
                TokenContent::<$string_type>::LessThan,
                TokenContent::<$string_type>::LessThanOrEqualTo,
                TokenContent::<$string_type>::Minus,
                TokenContent::<$string_type>::Newline,
                TokenContent::<$string_type>::NotEqualTo,
                TokenContent::<$string_type>::OpenBrace,
                TokenContent::<$string_type>::OpenParenthesis,
                TokenContent::<$string_type>::Plus,
                TokenContent::<$string_type>::Semicolon,
                TokenContent::<$string_type>::Slash,
            ] {
                assert!(matches!(
                    TokenCollection::try_from(
                        token_content.to_string().as_str()
                    ).map(|tokens| tokens.into_iter().collect::<Vec<_>>()).as_deref(),
                    Ok([Token { content: result_token_content, .. }])
                    if result_token_content.eq(&token_content)
                ));
            }
        };
    }

    assertion!(&str);
    assertion!(&'static str);
    assertion!(Arc<str>);
    assertion!(Box<str>);
    assertion!(Cow<'_, str>);
    assertion!(Rc<str>);
    assertion!(String);
}
