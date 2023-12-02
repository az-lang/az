from hypothesis import given

from az.tokenization import Token, tokenize_string

from tests.utils import split_any_string_keeping_separators, tokens_to_string
from . import strategies

NEWLINE = '\n'
UTF_8_ENCODING_NAME = 'utf-8'


@given(strategies.scripts_strings)
def test_basic(string: str) -> None:
    result = tokenize_string(string)

    assert isinstance(result, list)
    assert all(isinstance(element, Token) for element in result)


@given(strategies.scripts_strings)
def test_positions(string: str) -> None:
    result = tokenize_string(string)

    string_lines = split_any_string_keeping_separators(string, NEWLINE)
    byte_lines = split_any_string_keeping_separators(
        string.encode(UTF_8_ENCODING_NAME), NEWLINE.encode(UTF_8_ENCODING_NAME)
    )

    assert [
        token
        for token in result
        if (
            string_lines[token.position.start_line][
                int(token.position.start_character.utf_8) : int(
                    token.position.end_character.utf_8
                )
            ]
            if token.position.start_line == token.position.end_line
            else ''.join(
                [
                    string_lines[token.position.start_line][
                        int(token.position.start_character.utf_8) :
                    ],
                    *string_lines[
                        token.position.start_line + 1 : token.position.end_line
                    ],
                    string_lines[token.position.end_line][
                        : int(token.position.end_character.utf_8)
                    ],
                ]
            )
        )
        != token.content.string
    ] == []
    assert [
        token
        for token in result
        if (
            byte_lines[token.position.start_line][
                int(token.position.start_character.byte) : int(
                    token.position.end_character.byte
                )
            ]
            if token.position.start_line == token.position.end_line
            else b''.join(
                [
                    byte_lines[token.position.start_line][
                        int(token.position.start_character.byte) :
                    ],
                    *byte_lines[
                        token.position.start_line + 1 : token.position.end_line
                    ],
                    byte_lines[token.position.end_line][
                        : int(token.position.end_character.byte)
                    ],
                ]
            )
        )
        != token.content.string.encode(UTF_8_ENCODING_NAME)
    ] == []


@given(strategies.scripts_strings)
def test_round_trip(string: str) -> None:
    result = tokenize_string(string)

    assert tokens_to_string(result) == string
