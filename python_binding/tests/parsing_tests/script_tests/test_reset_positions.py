from az.parsing import Filler, FillerContent, FillerKind, Script
from az.tokenization import (
    ByteCount,
    CharacterPosition,
    SubstringPosition,
    TokenCollection,
    Utf8Count,
)
from hypothesis import given

from tests.strategies import script_strategy
from tests.tokenization_tests.utils import tokens_to_string


@given(script_strategy)
def test_basic(script: Script) -> None:
    result = script.reset_positions()  # type: ignore[func-returns-value]

    assert result is None


@given(script_strategy)
def test_round_trip(script: Script) -> None:
    script = Script(
        [],
        fillers=[
            Filler(
                FillerContent(FillerKind.NEWLINE),
                position=SubstringPosition(
                    start=CharacterPosition(
                        byte=ByteCount(0), utf_8=Utf8Count(0)
                    ),
                    end=CharacterPosition(
                        byte=ByteCount(1), utf_8=Utf8Count(1)
                    ),
                ),
            ),
            Filler(
                FillerContent(FillerKind.COMMENT_BLOCK, '/**/'),
                position=SubstringPosition(
                    start=CharacterPosition(
                        byte=ByteCount(1), utf_8=Utf8Count(1)
                    ),
                    end=CharacterPosition(
                        byte=ByteCount(4), utf_8=Utf8Count(4)
                    ),
                ),
            ),
        ],
    )
    round_tripped_script = Script.from_tokens(
        TokenCollection.from_string(tokens_to_string(script.tokenize()))
    )

    script.reset_positions()

    assert script == round_tripped_script
