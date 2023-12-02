from hypothesis import given

from az import tokenization
from az.tokenization import Token

from . import strategies


@given(strategies.tokens)
def test_round_trip(token: Token) -> None:
    result = repr(token)

    assert eval(result, dict(vars(tokenization))) == token
