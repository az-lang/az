from az.tokenization import Utf8Count
from hypothesis import given

from tests.utils import equivalence

from .strategies import utf_8_count_strategy


@given(utf_8_count_strategy)
def test_irreflexivity(utf_8_count: Utf8Count) -> None:
    assert not utf_8_count != utf_8_count  # noqa: SIM202


@given(utf_8_count_strategy, utf_8_count_strategy)
def test_symmetry(first: Utf8Count, second: Utf8Count) -> None:
    assert equivalence(first != second, second != first)


@given(utf_8_count_strategy, utf_8_count_strategy)
def test_equivalents(first: Utf8Count, second: Utf8Count) -> None:
    assert equivalence(first != second, first != second)
    assert equivalence(first != second, first > second or first < second)
    assert equivalence(first != second, first > second or second > first)
    assert equivalence(first != second, second < first or second > first)
    assert equivalence(first != second, second < first or first < second)
