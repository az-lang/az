from az.tokenization import TokenContent
from hypothesis import given

from tests.strategies import token_content_strategy
from tests.utils import equivalence, implication


@given(token_content_strategy)
def test_reflexivity(token_content: TokenContent) -> None:
    assert token_content == token_content


@given(token_content_strategy, token_content_strategy)
def test_symmetry(first: TokenContent, second: TokenContent) -> None:
    assert equivalence(first == second, second == first)


@given(token_content_strategy, token_content_strategy, token_content_strategy)
def test_transitivity(
    first: TokenContent, second: TokenContent, third: TokenContent
) -> None:
    assert implication(first == second and second == third, first == third)


@given(token_content_strategy, token_content_strategy)
def test_alternatives(first: TokenContent, second: TokenContent) -> None:
    assert equivalence(first == second, first == second)
