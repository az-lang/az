from hypothesis import given

from az.parsing import Statement, parse_tokens
from az.tokenization import PositionedToken

from . import strategies


@given(strategies.parseable_positioned_tokens_lists)
def test_basic(tokens: list[PositionedToken]) -> None:
    result = parse_tokens(tokens)

    assert isinstance(result, list)
    assert all(isinstance(element, Statement) for element in result)
