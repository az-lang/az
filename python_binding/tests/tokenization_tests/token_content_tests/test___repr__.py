from az import tokenization
from az.tokenization import TokenContent
from hypothesis import given

from tests.strategies import token_content_strategy


@given(token_content_strategy)
def test_round_trip(token_content: TokenContent) -> None:
    result = repr(token_content)

    assert eval(result, dict(vars(tokenization))) == token_content
