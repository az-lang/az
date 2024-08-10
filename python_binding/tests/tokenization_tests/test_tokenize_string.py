from az.tokenization import TokenCollection, TokenContent, TokenKind
from hypothesis import given

from .strategies import script_string_strategy
from .utils import tokens_to_string

NEWLINE = str(TokenContent(TokenKind.NEWLINE))
UTF_8_ENCODING_NAME = 'utf-8'


@given(script_string_strategy)
def test_basic(string: str) -> None:
    result = TokenCollection.from_string(string)

    assert isinstance(result, TokenCollection)


@given(script_string_strategy)
def test_positions(string: str) -> None:
    result = TokenCollection.from_string(string)

    byte_string = string.encode(UTF_8_ENCODING_NAME)

    assert [
        token
        for token in result
        if (
            string[
                int(token.position.start.utf_8) : int(token.position.end.utf_8)
            ]
            != str(token.content)
        )
    ] == []
    assert [
        token
        for token in result
        if (
            byte_string[
                int(token.position.start.byte) : int(token.position.end.byte)
            ]
            != str(token.content).encode(UTF_8_ENCODING_NAME)
        )
    ] == []


@given(script_string_strategy)
def test_round_trip(string: str) -> None:
    result = TokenCollection.from_string(string)

    assert tokens_to_string(result) == string
