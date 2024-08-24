pub(crate) const CONDITIONAL_ALTERNATIVE_OPENER: &str = "else";
pub(crate) const CONDITIONAL_ANTECEDENT_OPENER: &str = "if";
pub(crate) const FUNCTION_DEFINITION_OPENER: &str = "Function";
pub(crate) const RETURN_OPERATOR_STRING: &str = "return";
pub(crate) const WHILE_LOOP_OPENER: &str = "while";

const KEYWORDS: &[&str] = &[
    CONDITIONAL_ALTERNATIVE_OPENER,
    CONDITIONAL_ANTECEDENT_OPENER,
    FUNCTION_DEFINITION_OPENER,
    RETURN_OPERATOR_STRING,
    WHILE_LOOP_OPENER,
];

pub fn is_keyword(value: &str) -> bool {
    KEYWORDS.contains(&value)
}
