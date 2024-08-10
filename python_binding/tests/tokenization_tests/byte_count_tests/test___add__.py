import pytest
from az.tokenization import ByteCount
from hypothesis import given

from .strategies import (
    addable_byte_count_pair_strategy,
    addable_byte_count_triplet_strategy,
    byte_count_strategy,
    overflowing_addition_byte_count_pair_strategy,
    zero_byte_count_strategy,
)


@given(addable_byte_count_pair_strategy)
def test_basic(pair: tuple[ByteCount, ByteCount]) -> None:
    first, second = pair

    result = first + second

    assert isinstance(result, ByteCount)


@given(addable_byte_count_pair_strategy)
def test_commutativity(pair: tuple[ByteCount, ByteCount]) -> None:
    first, second = pair

    assert first + second == second + first


@given(byte_count_strategy, zero_byte_count_strategy)
def test_neutral_element(first: ByteCount, second: ByteCount) -> None:
    assert first + second == first == second + first


@given(addable_byte_count_triplet_strategy)
def test_associativity(
    triplet: tuple[ByteCount, ByteCount, ByteCount],
) -> None:
    first, second, third = triplet

    assert (first + second) + third == first + (second + third)


@given(overflowing_addition_byte_count_pair_strategy)
def test_overflow(pair: tuple[ByteCount, ByteCount]) -> None:
    first, second = pair

    with pytest.raises(OverflowError):
        first + second
