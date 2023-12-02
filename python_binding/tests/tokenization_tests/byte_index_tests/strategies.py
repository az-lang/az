from hypothesis import strategies as _st

from az.tokenization import ByteIndex as _ByteIndex

_MAX_BYTE_INDEX_VALUE = int(_ByteIndex.MAX)
byte_indices_values = _st.integers(0, _MAX_BYTE_INDEX_VALUE)
invalid_byte_indices_values = _st.integers(max_value=-1) | _st.integers(
    _MAX_BYTE_INDEX_VALUE + 1
)
byte_indices = _st.builds(_ByteIndex, byte_indices_values)
zero_byte_indices = _st.builds(_ByteIndex)


def _to_overflowing_addable_byte_indices(
    value: int,
) -> _st.SearchStrategy[tuple[_ByteIndex, _ByteIndex]]:
    assert 0 < value <= _MAX_BYTE_INDEX_VALUE
    return _st.tuples(
        _st.just(_ByteIndex(value)),
        _st.builds(
            _ByteIndex,
            _st.integers(
                _MAX_BYTE_INDEX_VALUE - value + 1, _MAX_BYTE_INDEX_VALUE
            ),
        ),
    )


def _to_addable_byte_indices(
    data: _st.DataObject, value: int, count: int
) -> tuple[_ByteIndex, ...]:
    values = [value]
    rest_increment = _MAX_BYTE_INDEX_VALUE - value
    for _ in range(count - 1):
        next_value = data.draw(_st.integers(0, rest_increment))
        values.append(next_value)
        rest_increment -= next_value
    return tuple(_ByteIndex(value) for value in values)


def _to_subtractable_byte_indices_pairs(
    data: _st.DataObject, value: int
) -> tuple[_ByteIndex, _ByteIndex]:
    return _ByteIndex(value), _ByteIndex(data.draw(_st.integers(0, value)))


def _to_overflowing_subtractable_byte_indices(
    value: int,
) -> _st.SearchStrategy[tuple[_ByteIndex, _ByteIndex]]:
    assert 0 <= value < _MAX_BYTE_INDEX_VALUE
    return _st.tuples(
        _st.just(_ByteIndex(value)),
        _st.builds(_ByteIndex, _st.integers(value + 1, _MAX_BYTE_INDEX_VALUE)),
    )


addable_byte_indices_pairs = _st.builds(
    _to_addable_byte_indices, _st.data(), byte_indices_values, _st.just(2)
)
addable_byte_indices_triplets = _st.builds(
    _to_addable_byte_indices, _st.data(), byte_indices_values, _st.just(3)
)
overflowing_addition_byte_indices_pairs = _st.integers(
    1, _MAX_BYTE_INDEX_VALUE
).flatmap(_to_overflowing_addable_byte_indices)


subtractable_byte_indices_pairs = _st.builds(
    _to_subtractable_byte_indices_pairs, _st.data(), byte_indices_values
)
overflowing_subtractition_byte_indices_pairs = _st.integers(
    0, _MAX_BYTE_INDEX_VALUE - 1
).flatmap(_to_overflowing_subtractable_byte_indices)
