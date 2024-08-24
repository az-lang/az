from az.tokenization import TokenContent, TokenKind
from hypothesis import given

from tests.strategies import (
    token_content_strategy,
    token_kind_with_state_strategy,
)


@given(token_kind_with_state_strategy)
def test_basic(token_kind_with_state: tuple[TokenKind, str | None]) -> None:
    token_kind, state = token_kind_with_state

    result = TokenContent(token_kind, state)

    assert isinstance(result, TokenContent)
    assert result.kind is token_kind
    assert result.state == state


@given(token_content_strategy)
def test_round_trip(token_content: TokenContent) -> None:
    result = TokenContent(token_content.kind, token_content.state)

    assert result == token_content
