from hypothesis import strategies as _st

from az.tokenization import Utf8Index as _Utf8Index

_MAX_UTF8_INDEX_VALUE = int(_Utf8Index.MAX)
utf8_indices_values = _st.integers(0, _MAX_UTF8_INDEX_VALUE)
invalid_utf8_indices_values = _st.integers(max_value=-1) | _st.integers(
    _MAX_UTF8_INDEX_VALUE + 1
)
utf8_indices = _st.builds(_Utf8Index, utf8_indices_values)
zero_utf8_indices = _st.builds(_Utf8Index)


def _to_overflowing_addable_utf8_indices(
    value: int,
) -> _st.SearchStrategy[tuple[_Utf8Index, _Utf8Index]]:
    assert 0 < value <= _MAX_UTF8_INDEX_VALUE
    return _st.tuples(
        _st.just(_Utf8Index(value)),
        _st.builds(
            _Utf8Index,
            _st.integers(
                _MAX_UTF8_INDEX_VALUE - value + 1, _MAX_UTF8_INDEX_VALUE
            ),
        ),
    )


def _to_addable_utf8_indices(
    data: _st.DataObject, value: int, count: int
) -> tuple[_Utf8Index, ...]:
    values = [value]
    rest_increment = _MAX_UTF8_INDEX_VALUE - value
    for _ in range(count - 1):
        next_value = data.draw(_st.integers(0, rest_increment))
        values.append(next_value)
        rest_increment -= next_value
    return tuple(_Utf8Index(value) for value in values)


def _to_subtractable_utf8_indices_pairs(
    data: _st.DataObject, value: int
) -> tuple[_Utf8Index, _Utf8Index]:
    return _Utf8Index(value), _Utf8Index(data.draw(_st.integers(0, value)))


def _to_overflowing_subtractable_utf8_indices(
    value: int,
) -> _st.SearchStrategy[tuple[_Utf8Index, _Utf8Index]]:
    assert 0 <= value < _MAX_UTF8_INDEX_VALUE
    return _st.tuples(
        _st.just(_Utf8Index(value)),
        _st.builds(_Utf8Index, _st.integers(value + 1, _MAX_UTF8_INDEX_VALUE)),
    )


addable_utf8_indices_pairs = _st.builds(
    _to_addable_utf8_indices, _st.data(), utf8_indices_values, _st.just(2)
)
addable_utf8_indices_triplets = _st.builds(
    _to_addable_utf8_indices, _st.data(), utf8_indices_values, _st.just(3)
)
overflowing_addition_utf8_indices_pairs = _st.integers(
    1, _MAX_UTF8_INDEX_VALUE
).flatmap(_to_overflowing_addable_utf8_indices)


subtractable_utf8_indices_pairs = _st.builds(
    _to_subtractable_utf8_indices_pairs, _st.data(), utf8_indices_values
)
overflowing_subtractition_utf8_indices_pairs = _st.integers(
    0, _MAX_UTF8_INDEX_VALUE - 1
).flatmap(_to_overflowing_subtractable_utf8_indices)
