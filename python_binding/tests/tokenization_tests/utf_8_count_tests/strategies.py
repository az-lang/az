from az.tokenization import Utf8Count as _Utf8Count
from hypothesis import strategies as _st

_MAX_UTF_8_COUNT_VALUE = int(_Utf8Count.MAX)
_MIN_UTF_8_COUNT_VALUE = int(_Utf8Count.MIN)
utf_8_count_value_strategy = _st.integers(
    _MIN_UTF_8_COUNT_VALUE, _MAX_UTF_8_COUNT_VALUE
)
invalid_utf_8_count_value_strategy = _st.integers(
    max_value=_MIN_UTF_8_COUNT_VALUE - 1
) | _st.integers(_MAX_UTF_8_COUNT_VALUE + 1)
utf_8_count_strategy = _st.builds(_Utf8Count, utf_8_count_value_strategy)
zero_utf_8_count_strategy = _st.builds(_Utf8Count)


@_st.composite
def _to_overflowing_addable_utf_8_count_pair_strategy(
    draw: _st.DrawFn, values: _st.SearchStrategy[int], /
) -> tuple[_Utf8Count, _Utf8Count]:
    value = draw(values)
    assert _MIN_UTF_8_COUNT_VALUE < value <= _MAX_UTF_8_COUNT_VALUE
    return (
        _Utf8Count(value),
        _Utf8Count(
            draw(
                _st.integers(
                    _MAX_UTF_8_COUNT_VALUE - value + 1, _MAX_UTF_8_COUNT_VALUE
                )
            )
        ),
    )


def _to_addable_utf_8_indices(
    data: _st.DataObject, value: int, count: int
) -> tuple[_Utf8Count, ...]:
    values = [value]
    rest_increment = _MAX_UTF_8_COUNT_VALUE - value
    for _ in range(count - 1):
        next_value = data.draw(
            _st.integers(_MIN_UTF_8_COUNT_VALUE, rest_increment)
        )
        values.append(next_value)
        rest_increment -= next_value
    return tuple(_Utf8Count(value) for value in values)


def _to_subtractable_utf_8_count_pair(
    data: _st.DataObject, value: int, /
) -> tuple[_Utf8Count, _Utf8Count]:
    return _Utf8Count(value), _Utf8Count(
        data.draw(_st.integers(_MIN_UTF_8_COUNT_VALUE, value))
    )


@_st.composite
def _to_overflowing_subtractable_utf_8_count_pair_strategy(
    draw: _st.DrawFn, values: _st.SearchStrategy[int], /
) -> tuple[_Utf8Count, _Utf8Count]:
    value = draw(values)
    assert _MIN_UTF_8_COUNT_VALUE <= value < _MAX_UTF_8_COUNT_VALUE
    return (
        _Utf8Count(value),
        _Utf8Count(draw(_st.integers(value + 1, _MAX_UTF_8_COUNT_VALUE))),
    )


addable_utf_8_count_pair_strategy = _st.builds(
    _to_addable_utf_8_indices,
    _st.data(),
    utf_8_count_value_strategy,
    _st.just(2),
)
addable_utf_8_count_triplet_strategy = _st.builds(
    _to_addable_utf_8_indices,
    _st.data(),
    utf_8_count_value_strategy,
    _st.just(3),
)
overflowing_addition_utf_8_count_pair_strategy = (
    _to_overflowing_addable_utf_8_count_pair_strategy(
        _st.integers(_MIN_UTF_8_COUNT_VALUE + 1, _MAX_UTF_8_COUNT_VALUE)
    )
)

subtractable_utf_8_count_pair_strategy = _st.builds(
    _to_subtractable_utf_8_count_pair, _st.data(), utf_8_count_value_strategy
)
overflowing_subtraction_utf_8_count_pair_strategy = (
    _to_overflowing_subtractable_utf_8_count_pair_strategy(
        _st.integers(_MIN_UTF_8_COUNT_VALUE, _MAX_UTF_8_COUNT_VALUE - 1)
    )
)
