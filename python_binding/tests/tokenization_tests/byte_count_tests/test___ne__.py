from az.tokenization import ByteCount
from hypothesis import given

from tests.utils import equivalence

from .strategies import byte_count_strategy


@given(byte_count_strategy)
def test_irreflexivity(byte_count: ByteCount) -> None:
    assert not byte_count != byte_count  # noqa: SIM202


@given(byte_count_strategy, byte_count_strategy)
def test_symmetry(first: ByteCount, second: ByteCount) -> None:
    assert equivalence(first != second, second != first)


@given(byte_count_strategy, byte_count_strategy)
def test_equivalents(first: ByteCount, second: ByteCount) -> None:
    assert equivalence(first != second, first != second)
    assert equivalence(first != second, first > second or first < second)
    assert equivalence(first != second, first > second or second > first)
    assert equivalence(first != second, second < first or second > first)
    assert equivalence(first != second, second < first or first < second)
