from hypothesis import given

from az import tokenization
from az.tokenization import TokenContent

from . import strategies


@given(strategies.tokens_contents)
def test_round_trip(token_content: TokenContent) -> None:
    result = repr(token_content)

    assert eval(result, dict(vars(tokenization))) == token_content
