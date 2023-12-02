use proptest::prelude::proptest;
use proptest::string::string_regex;
use proptest::test_runner::{Config, TestRunner};

use az::tokenization::{
    NumericLiteralType, PositionedToken, Token, TryTokenize,
};

const DIGIT_PATTERN: &str = r"\d";

fn to_floating_point_value_pattern() -> String {
    format!(
        r"({digit}+(\.{digit}*)?([eE][+-]?{digit}+)?|\.{digit}+([eE][+-]?{digit}+)?)",
        digit = DIGIT_PATTERN
    )
}

fn to_integer_value_pattern() -> String {
    format!(r"{}+", DIGIT_PATTERN)
}

#[test]
fn test_empty() {
    assert_eq!("".try_tokenize(), Ok(vec![]));
}

proptest! {
    #[test]
    fn test_doesnt_crash(text in r".*") {
        drop(text.try_tokenize());
    }

    #[test]
    fn test_comment_block(text in r"/*[^\*]*\*/") {
        matches!(
            text.try_tokenize().as_deref(),
            Ok([PositionedToken { token: Token::CommentBlock(_), .. }])
        );
    }

    #[test]
    fn test_comment_line(text in r"//[^/\n]*\n?") {
        matches!(
            text.try_tokenize().as_deref(),
            Ok([PositionedToken { token: Token::CommentLine(_), .. }])
        );
    }

    #[test]
    fn test_identifier(text in r"[a-zA-Z_][a-zA-Z0-9_]*") {
        matches!(
            text.try_tokenize().as_deref(),
            Ok([PositionedToken { token: Token::Identifier(_), .. }])
        );
    }

    #[test]
    fn test_whitespace(text in r"\s+") {
        matches!(
            text.try_tokenize().as_deref(),
            Ok([PositionedToken { token: Token::Whitespace(_), .. }])
        );
    }
}

#[test]
fn test_floating_point_literals() {
    for floating_point_type in
        [NumericLiteralType::F32, NumericLiteralType::F64]
    {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &string_regex(&format!(
                    r"(?-u){}_{}",
                    to_floating_point_value_pattern(),
                    floating_point_type
                ))
                .unwrap(),
                |text| {
                    assert!(matches!(
                        text.try_tokenize().as_deref(),
                        Ok([PositionedToken {
                            token: Token::NumericLiteral { value, type_ },
                            ..
                        }])
                        if (
                            text.starts_with(value)
                            && text.chars().nth(value.len()).unwrap() == '_'
                            && type_.eq(&floating_point_type)
                        )
                    ));
                    Ok(())
                },
            )
            .unwrap();
    }
}

#[test]
fn test_integer_literals() {
    for integer_type in [
        NumericLiteralType::I8,
        NumericLiteralType::I16,
        NumericLiteralType::I32,
        NumericLiteralType::I64,
        NumericLiteralType::ISize,
        NumericLiteralType::U8,
        NumericLiteralType::U16,
        NumericLiteralType::U32,
        NumericLiteralType::U64,
        NumericLiteralType::USize,
    ] {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &string_regex(&format!(
                    r"(?-u){}_{}",
                    to_integer_value_pattern(),
                    integer_type
                ))
                .unwrap(),
                |text| {
                    assert!(matches!(
                        text.try_tokenize().as_deref(),
                        Ok([PositionedToken {
                            token: Token::NumericLiteral { value, type_ },
                            ..
                        }])
                        if (
                            text.starts_with(value)
                            && text.chars().nth(value.len()).unwrap() == '_'
                            && type_.eq(&integer_type)
                        )
                    ));
                    Ok(())
                },
            )
            .unwrap();
    }
}

#[test]
fn test_stateless_tokens() {
    for token in [
        Token::Arrow,
        Token::Assignment,
        Token::Asterisk,
        Token::CloseBrace,
        Token::CloseParenthesis,
        Token::Colon,
        Token::Comma,
        Token::Dot,
        Token::EqualTo,
        Token::GreaterThan,
        Token::GreaterThanOrEqualTo,
        Token::LowerThan,
        Token::LowerThanOrEqualTo,
        Token::Minus,
        Token::Newline,
        Token::NotEqualTo,
        Token::OpenBrace,
        Token::OpenParenthesis,
        Token::Plus,
        Token::Semicolon,
        Token::Slash,
    ] {
        assert!(matches!(
            token
                .to_string()
                .as_str()
                .try_tokenize()
                .as_deref(),
            Ok([PositionedToken { token: result_token, .. }])
            if result_token.eq(&token)
        ));
    }
}
