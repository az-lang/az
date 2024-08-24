from az.tokenization import SubstringPosition, Token, TokenContent
from hypothesis import given

from tests.strategies import (
    substring_position_strategy,
    token_content_strategy,
    token_strategy,
)


@given(token_content_strategy, substring_position_strategy)
def test_basic(content: TokenContent, position: SubstringPosition) -> None:
    result = Token(content, position=position)

    assert isinstance(result, Token)
    assert result.content == content
    assert result.position == position


@given(token_strategy)
def test_round_trip(token: Token) -> None:
    result = Token(token.content, position=token.position)

    assert result == token
