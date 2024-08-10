import pytest
from az.tokenization import ByteCount
from hypothesis import given

from .strategies import (
    byte_count_strategy,
    overflowing_subtraction_byte_count_pair_strategy,
    subtractable_byte_count_pair_strategy,
    zero_byte_count_strategy,
)


@given(subtractable_byte_count_pair_strategy)
def test_basic(pair: tuple[ByteCount, ByteCount]) -> None:
    first, second = pair

    result = first - second

    assert isinstance(result, ByteCount)


@given(subtractable_byte_count_pair_strategy)
def test_commutative_case(pair: tuple[ByteCount, ByteCount]) -> None:
    first, second = pair

    if first == second:
        assert first - second == second - first
    else:
        with pytest.raises(OverflowError):
            first - second
            second - first


@given(byte_count_strategy, zero_byte_count_strategy)
def test_diagonal(byte_count: ByteCount, zero_byte_count: ByteCount) -> None:
    assert byte_count - byte_count == zero_byte_count


@given(byte_count_strategy, zero_byte_count_strategy)
def test_right_neutral_element(
    byte_count: ByteCount, zero_byte_count: ByteCount
) -> None:
    assert byte_count - zero_byte_count == byte_count


@given(overflowing_subtraction_byte_count_pair_strategy)
def test_overflow(pair: tuple[ByteCount, ByteCount]) -> None:
    first, second = pair

    with pytest.raises(OverflowError):
        first - second
