from az.tokenization import ByteCount
from hypothesis import given

from tests.utils import equivalence, implication

from .strategies import byte_count_strategy


@given(byte_count_strategy)
def test_reflexivity(byte_count: ByteCount) -> None:
    assert byte_count <= byte_count


@given(byte_count_strategy, byte_count_strategy)
def test_antisymmetry(first: ByteCount, second: ByteCount) -> None:
    assert equivalence(first <= second <= first, first == second)


@given(byte_count_strategy, byte_count_strategy, byte_count_strategy)
def test_transitivity(
    first: ByteCount, second: ByteCount, third: ByteCount
) -> None:
    assert implication(first <= second <= third, first <= third)


@given(byte_count_strategy, byte_count_strategy)
def test_alternatives(first: ByteCount, second: ByteCount) -> None:
    assert equivalence(first <= second, first < second or first == second)
    assert equivalence(first <= second, second > first or first == second)
    assert equivalence(first <= second, first < second or first == second)
    assert equivalence(first <= second, second > first or first == second)
    assert equivalence(first <= second, second >= first)
    assert equivalence(first <= second, not second < first)
    assert equivalence(first <= second, not first > second)
