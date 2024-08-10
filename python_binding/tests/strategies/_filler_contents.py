from az.parsing import (
    FillerContent as _FillerContent,
    FillerKind as _FillerKind,
)
from hypothesis import strategies as _st

from tests.strategies._token_content_strings import (
    comment_block_string_strategy as _comment_block_strings,
    comment_line_string_strategy as _comment_line_strings,
    whitespace_string_strategy as _whitespace_strings,
)

_comment_filler_contents = _st.builds(
    _FillerContent, _st.just(_FillerKind.COMMENT_LINE), _comment_line_strings
) | _st.builds(
    _FillerContent, _st.just(_FillerKind.COMMENT_BLOCK), _comment_block_strings
)
non_comment_filler_content_strategy = _st.builds(
    _FillerContent, _st.just(_FillerKind.NEWLINE)
) | _st.builds(
    _FillerContent, _st.just(_FillerKind.WHITESPACE), _whitespace_strings
)
filler_kind_with_state_strategy = (
    _st.tuples(_st.just(_FillerKind.NEWLINE), _st.none())
    | _st.tuples(_st.just(_FillerKind.WHITESPACE), _whitespace_strings)
    | _st.tuples(_st.just(_FillerKind.COMMENT_LINE), _comment_line_strings)
    | _st.tuples(_st.just(_FillerKind.COMMENT_BLOCK), _comment_block_strings)
)
filler_content_strategy = (
    non_comment_filler_content_strategy | _comment_filler_contents
)
