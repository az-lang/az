pub(crate) fn to_floating_point_value_pattern() -> String {
    format!(
        r"(({non_zero_digit}{digit}*)(\.{digit}*)?([eE][+-]?{digit}+)?|\.{digit}+([eE][+-]?{digit}+)?)",
        digit = DIGIT_PATTERN,
        non_zero_digit = NON_ZERO_DIGIT_PATTERN,
    )
}

pub(crate) fn to_integer_value_pattern() -> String {
    format!(r"{}{}*", NON_ZERO_DIGIT_PATTERN, DIGIT_PATTERN)
}

pub(super) const COMMENT_BLOCK_PATTERN: &str = r"/\*[^\*]*\*/";
pub(super) const COMMENT_LINE_PATTERN: &str = r"//[^/\n]*\n";
pub(super) const IDENTIFIER_PATTERN: &str = r"[a-zA-Z_][a-zA-Z0-9_]*";
pub(super) const WHITESPACE_PATTERN: &str = r"[^\S\n\r]+";

const DIGIT_PATTERN: &str = r"[0-9]";
const NON_ZERO_DIGIT_PATTERN: &str = r"[1-9]";
