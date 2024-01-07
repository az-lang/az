from hypothesis import given

from az.tokenization import PositionedToken, tokenize_string

from tests.utils import tokens_to_string
from . import strategies


@given(strategies.strings)
def test_basic(string: str) -> None:
    result = tokenize_string(string)

    assert isinstance(result, list)
    assert all(isinstance(element, PositionedToken) for element in result)


@given(strategies.strings)
def test_round_trip(string: str) -> None:
    result = tokenize_string(string)

    assert tokens_to_string(result) == string
