from az.tokenization import ByteCount
from hypothesis import given

from tests.utils import equivalence, implication

from .strategies import byte_count_strategy


@given(byte_count_strategy)
def test_irreflexivity(byte_count: ByteCount) -> None:
    assert not byte_count > byte_count


@given(byte_count_strategy, byte_count_strategy)
def test_asymmetry(first: ByteCount, second: ByteCount) -> None:
    assert implication(first > second, not second > first)


@given(byte_count_strategy, byte_count_strategy, byte_count_strategy)
def test_transitivity(
    first: ByteCount, second: ByteCount, third: ByteCount
) -> None:
    assert implication(first > second > third, first > third)


@given(byte_count_strategy, byte_count_strategy)
def test_alternatives(first: ByteCount, second: ByteCount) -> None:
    assert equivalence(first > second, first >= second and first != second)
    assert equivalence(first > second, first >= second and first != second)
    assert equivalence(first > second, second <= first and first != second)
    assert equivalence(first > second, second <= first and first != second)
    assert equivalence(first > second, second < first)
    assert equivalence(first > second, not second >= first)
    assert equivalence(first > second, not first <= second)
