use proptest::prelude::{prop, Strategy};

use az::parsing::KEYWORDS;

use super::patterns::{
    COMMENT_BLOCK_PATTERN, COMMENT_LINE_PATTERN, IDENTIFIER_PATTERN,
    WHITESPACE_PATTERN,
};

pub(crate) fn to_comment_line_string_strategy() -> impl Strategy<Value = String>
{
    prop::string::string_regex(COMMENT_LINE_PATTERN).unwrap()
}

pub(crate) fn to_comment_block_string_strategy(
) -> impl Strategy<Value = String> {
    prop::string::string_regex(COMMENT_BLOCK_PATTERN).unwrap()
}

pub(crate) fn to_identifier_string_strategy() -> impl Strategy<Value = String>
{
    prop::string::string_regex(IDENTIFIER_PATTERN)
        .unwrap()
        .prop_filter("Keywords are not valid identifiers", |value| {
            !KEYWORDS.contains(&value.as_str())
        })
}

pub(crate) fn to_whitespace_string_strategy() -> impl Strategy<Value = String>
{
    prop::string::string_regex(WHITESPACE_PATTERN).unwrap()
}
