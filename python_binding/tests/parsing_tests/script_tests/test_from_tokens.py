import typing as t
from itertools import takewhile

from hypothesis import given

from az.parsing import Filler, FillerContent, FillerKind, Script, Statement
from az.tokenization import Token, TokenKind

from tests.utils import script_to_fillers
from . import strategies


@given(strategies.parseable_tokens_lists)
def test_basic(tokens: list[Token]) -> None:
    result = Script.from_tokens(tokens)

    assert isinstance(result, Script)
    assert all(isinstance(element, Statement) for element in result.statements)
    assert all(isinstance(element, Filler) for element in result.fillers)


@given(strategies.parseable_tokens_lists)
def test_size(tokens: list[Token]) -> None:
    result = Script.from_tokens(tokens)

    filler_tokens_count = to_filler_tokens_count(tokens)
    assert (
        bool(len(tokens) - filler_tokens_count)
        <= len(result.statements)
        <= len(tokens) - filler_tokens_count
    )
    assert len(result.fillers) <= filler_tokens_count
    assert len(result.fillers) == len(
        list(takewhile(is_filler_token, reversed(tokens)))
    )


@given(strategies.parseable_tokens_lists)
def test_fillers_preservation(tokens: list[Token]) -> None:
    result = Script.from_tokens(tokens)

    assert script_to_fillers(result) == tokens_to_fillers(tokens)


def to_filler_tokens_count(tokens: t.Iterable[Token]) -> int:
    return sum(1 for token in tokens if is_filler_token(token))


def is_filler_token(token: Token) -> bool:
    token_kind = token.content.kind
    return (
        token_kind == TokenKind.COMMENT_BLOCK
        or token_kind == TokenKind.COMMENT_LINE
        or token_kind == TokenKind.NEWLINE
        or token_kind == TokenKind.WHITESPACE
    )


def tokens_to_fillers(tokens: list[Token]) -> list[Filler]:
    return [
        Filler(
            content=FillerContent(
                filler_token_kind_to_filler_kind(token.content.kind),
                token.content.string,
            ),
            position=token.position,
        )
        for token in tokens
        if is_filler_token(token)
    ]


def filler_token_kind_to_filler_kind(token_kind: TokenKind) -> FillerKind:
    if token_kind == TokenKind.COMMENT_BLOCK:
        return FillerKind.COMMENT_BLOCK
    elif token_kind == TokenKind.COMMENT_LINE:
        return FillerKind.COMMENT_LINE
    elif token_kind == TokenKind.NEWLINE:
        return FillerKind.NEWLINE
    elif token_kind == TokenKind.WHITESPACE:
        return FillerKind.WHITESPACE
    else:
        raise ValueError(token_kind)
