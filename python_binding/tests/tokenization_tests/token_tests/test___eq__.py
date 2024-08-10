from az.tokenization import Token
from hypothesis import given

from tests.strategies import token_strategy
from tests.utils import equivalence, implication


@given(token_strategy)
def test_reflexivity(token: Token) -> None:
    assert token == token


@given(token_strategy, token_strategy)
def test_symmetry(first: Token, second: Token) -> None:
    assert equivalence(first == second, second == first)


@given(token_strategy, token_strategy, token_strategy)
def test_transitivity(first: Token, second: Token, third: Token) -> None:
    assert implication(first == second and second == third, first == third)


@given(token_strategy, token_strategy)
def test_alternatives(first: Token, second: Token) -> None:
    assert equivalence(first == second, first == second)
