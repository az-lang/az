from az.tokenization import ByteCount as _ByteCount
from hypothesis import strategies as _st

_MAX_BYTE_COUNT_VALUE = int(_ByteCount.MAX)
_MIN_BYTE_COUNT_VALUE = int(_ByteCount.MIN)
byte_count_value_strategy = _st.integers(
    _MIN_BYTE_COUNT_VALUE, _MAX_BYTE_COUNT_VALUE
)
invalid_byte_count_value_strategy = _st.integers(
    max_value=_MIN_BYTE_COUNT_VALUE - 1
) | _st.integers(_MAX_BYTE_COUNT_VALUE + 1)
byte_count_strategy = _st.builds(_ByteCount, byte_count_value_strategy)
zero_byte_count_strategy = _st.builds(_ByteCount)


@_st.composite
def _to_overflowing_addable_byte_count_pair_strategy(
    draw: _st.DrawFn, values: _st.SearchStrategy[int], /
) -> tuple[_ByteCount, _ByteCount]:
    value = draw(values)
    assert _MIN_BYTE_COUNT_VALUE < value <= _MAX_BYTE_COUNT_VALUE
    return (
        _ByteCount(value),
        _ByteCount(
            draw(
                _st.integers(
                    _MAX_BYTE_COUNT_VALUE - value + 1, _MAX_BYTE_COUNT_VALUE
                )
            )
        ),
    )


def _to_addable_byte_indices(
    data: _st.DataObject, value: int, count: int
) -> tuple[_ByteCount, ...]:
    values = [value]
    rest_increment = _MAX_BYTE_COUNT_VALUE - value
    for _ in range(count - 1):
        next_value = data.draw(
            _st.integers(_MIN_BYTE_COUNT_VALUE, rest_increment)
        )
        values.append(next_value)
        rest_increment -= next_value
    return tuple(_ByteCount(value) for value in values)


def _to_subtractable_byte_count_pair(
    data: _st.DataObject, value: int, /
) -> tuple[_ByteCount, _ByteCount]:
    return _ByteCount(value), _ByteCount(
        data.draw(_st.integers(_MIN_BYTE_COUNT_VALUE, value))
    )


@_st.composite
def _to_overflowing_subtractable_byte_count_pair_strategy(
    draw: _st.DrawFn, values: _st.SearchStrategy[int], /
) -> tuple[_ByteCount, _ByteCount]:
    value = draw(values)
    assert _MIN_BYTE_COUNT_VALUE <= value < _MAX_BYTE_COUNT_VALUE
    return (
        _ByteCount(value),
        _ByteCount(draw(_st.integers(value + 1, _MAX_BYTE_COUNT_VALUE))),
    )


addable_byte_count_pair_strategy = _st.builds(
    _to_addable_byte_indices,
    _st.data(),
    byte_count_value_strategy,
    _st.just(2),
)
addable_byte_count_triplet_strategy = _st.builds(
    _to_addable_byte_indices,
    _st.data(),
    byte_count_value_strategy,
    _st.just(3),
)
overflowing_addition_byte_count_pair_strategy = (
    _to_overflowing_addable_byte_count_pair_strategy(
        _st.integers(_MIN_BYTE_COUNT_VALUE + 1, _MAX_BYTE_COUNT_VALUE)
    )
)

subtractable_byte_count_pair_strategy = _st.builds(
    _to_subtractable_byte_count_pair, _st.data(), byte_count_value_strategy
)
overflowing_subtraction_byte_count_pair_strategy = (
    _to_overflowing_subtractable_byte_count_pair_strategy(
        _st.integers(_MIN_BYTE_COUNT_VALUE, _MAX_BYTE_COUNT_VALUE - 1)
    )
)
