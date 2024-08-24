from az.tokenization import Utf8Count
from hypothesis import given

from tests.utils import equivalence, implication

from .strategies import utf_8_count_strategy


@given(utf_8_count_strategy)
def test_reflexivity(utf_8_count: Utf8Count) -> None:
    assert utf_8_count == utf_8_count


@given(utf_8_count_strategy, utf_8_count_strategy)
def test_symmetry(first: Utf8Count, second: Utf8Count) -> None:
    assert equivalence(first == second, second == first)


@given(utf_8_count_strategy, utf_8_count_strategy, utf_8_count_strategy)
def test_transitivity(
    first: Utf8Count, second: Utf8Count, third: Utf8Count
) -> None:
    assert implication(first == second and second == third, first == third)


@given(utf_8_count_strategy, utf_8_count_strategy)
def test_alternatives(first: Utf8Count, second: Utf8Count) -> None:
    assert equivalence(first == second, first == second)
    assert equivalence(first == second, first >= second and first <= second)
    assert equivalence(first == second, first >= second and second >= first)
    assert equivalence(first == second, second <= first and second >= first)
    assert equivalence(first == second, second <= first and first <= second)
