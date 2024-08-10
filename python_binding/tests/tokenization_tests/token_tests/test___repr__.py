from az import tokenization
from az.tokenization import Token
from hypothesis import given

from tests.strategies import token_strategy


@given(token_strategy)
def test_round_trip(token: Token) -> None:
    result = repr(token)

    assert eval(result, dict(vars(tokenization))) == token
