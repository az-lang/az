from hypothesis import given

from az.tokenization import TokenContent

from tests.utils import equivalence, implication
from . import strategies


@given(strategies.tokens_contents)
def test_reflexivity(token_content: TokenContent) -> None:
    assert token_content == token_content


@given(strategies.tokens_contents, strategies.tokens_contents)
def test_symmetry(first: TokenContent, second: TokenContent) -> None:
    assert equivalence(first == second, second == first)


@given(
    strategies.tokens_contents,
    strategies.tokens_contents,
    strategies.tokens_contents,
)
def test_transitivity(
    first: TokenContent, second: TokenContent, third: TokenContent
) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.tokens_contents, strategies.tokens_contents)
def test_alternatives(first: TokenContent, second: TokenContent) -> None:
    assert equivalence(first == second, first == second)
