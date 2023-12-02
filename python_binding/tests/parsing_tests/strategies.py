from hypothesis import strategies as _st

from az.tokenization import tokenize_string as _tokenize_string

from tests import strategies as _strategies

parseable_positioned_tokens_lists = _st.builds(
    _tokenize_string, _strategies.strings
)
