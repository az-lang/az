from az.tokenization import TokenKind
from hypothesis import given

from tests.strategies import token_kind_strategy


@given(token_kind_strategy)
def test_round_trip(token_kind: TokenKind) -> None:
    result = repr(token_kind)

    assert eval(result) is token_kind
