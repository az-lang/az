from hypothesis import given

from az.tokenization import Token

from tests.utils import equivalence, implication
from . import strategies


@given(strategies.tokens)
def test_reflexivity(token: Token) -> None:
    assert token == token


@given(strategies.tokens, strategies.tokens)
def test_symmetry(first: Token, second: Token) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.tokens, strategies.tokens, strategies.tokens)
def test_transitivity(first: Token, second: Token, third: Token) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.tokens, strategies.tokens)
def test_alternatives(first: Token, second: Token) -> None:
    assert equivalence(first == second, first == second)
