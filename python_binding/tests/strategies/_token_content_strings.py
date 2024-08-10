from __future__ import annotations

import string
from operator import add

from hypothesis import strategies as _st

comment_line_string_strategy = _st.from_regex(r'^//[^\n]*$', fullmatch=True)
comment_block_string_strategy = _st.from_regex(
    r'^/\*([^\*]*\*+[^*/])*[^\*]*\*/$', fullmatch=True
)
whitespace_string_strategy = _st.text(
    _st.characters(categories=['Zs']), min_size=1
)

non_starting_digit_string_strategy = _st.text(string.digits)
integer_literal_value_strategy = _st.just('0') | _st.builds(
    add,
    _st.sampled_from(string.digits[1:]),
    non_starting_digit_string_strategy,
)
non_empty_non_starting_digit_string_strategy = _st.text(
    string.digits, min_size=1
)
_floating_point_literal_value_strategies = [
    integer_literal_value_strategy,
    _st.builds(
        add,
        integer_literal_value_strategy.map(lambda value: value + '.'),
        non_starting_digit_string_strategy,
    ),
    _st.builds(
        add, _st.just('.'), non_empty_non_starting_digit_string_strategy
    ),
]
_floating_point_literal_value_strategies += [
    _st.tuples(
        variant,
        _st.sampled_from('eE'),
        _st.sampled_from(['+', '-', '']),
        non_empty_non_starting_digit_string_strategy,
    ).map(''.join)
    for variant in _floating_point_literal_value_strategies
]
floating_point_literal_value_strategy = _st.one_of(
    _floating_point_literal_value_strategies
)
identifier_token_string_strategy = _st.from_regex(
    r'^[a-zA-Z_][a-zA-Z0-9_]*$', fullmatch=True
)
