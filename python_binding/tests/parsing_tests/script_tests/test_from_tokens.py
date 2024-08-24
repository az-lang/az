from collections.abc import Iterable

from az.parsing import Filler, FillerContent, FillerKind, Script
from az.tokenization import Token, TokenCollection, TokenContent, TokenKind
from hypothesis import given

from tests.parsing_tests.utils import script_to_fillers
from tests.strategies import parseable_token_collection_strategy


@given(parseable_token_collection_strategy)
def test_basic(tokens: TokenCollection) -> None:
    result = Script.from_tokens(tokens)

    assert isinstance(result, Script)
    assert result.tokenize() == tokens


@given(parseable_token_collection_strategy)
def test_fillers_preservation(tokens: TokenCollection) -> None:
    result = Script.from_tokens(tokens)

    assert script_to_fillers(result) == tokens_to_fillers(tokens)


def is_filler_token(token: Token) -> bool:
    token_kind = token.content.kind
    return (
        token_kind is TokenKind.COMMENT_BLOCK
        or token_kind is TokenKind.COMMENT_LINE
        or token_kind is TokenKind.NEWLINE
        or token_kind is TokenKind.WHITESPACE
    )


def tokens_to_fillers(tokens: Iterable[Token], /) -> list[Filler]:
    return [
        Filler(
            token_content_to_filler_content(token.content),
            position=token.position,
        )
        for token in tokens
        if is_filler_token(token)
    ]


def token_content_to_filler_content(
    token_content: TokenContent, /
) -> FillerContent:
    return FillerContent(
        token_kind_to_filler_kind(token_content.kind), token_content.state
    )


def token_kind_to_filler_kind(token_kind: TokenKind) -> FillerKind:
    if token_kind is TokenKind.COMMENT_BLOCK:
        return FillerKind.COMMENT_BLOCK
    elif token_kind is TokenKind.COMMENT_LINE:
        return FillerKind.COMMENT_LINE
    elif token_kind is TokenKind.NEWLINE:
        return FillerKind.NEWLINE
    elif token_kind is TokenKind.WHITESPACE:
        return FillerKind.WHITESPACE
    else:
        raise ValueError(token_kind)
