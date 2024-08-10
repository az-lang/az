from az.tokenization import TokenKind
from hypothesis import given

from tests.strategies import token_kind_strategy
from tests.utils import pickling_round_trip


@given(token_kind_strategy)
def test_round_trip(token_kind: TokenKind) -> None:
    assert pickling_round_trip(token_kind) is token_kind
