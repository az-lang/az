pub(super) fn range_to_containment_string(
    range: &std::ops::RangeInclusive<usize>,
) -> String {
    if range.start() == range.end() {
        format!("equal to {}", range.start())
    } else if range.end() - range.start() == 1usize {
        format!("either {} or {}", range.start(), range.end())
    } else {
        format!("from {}, ..., {}", range.start(), range.end())
    }
}
