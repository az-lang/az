from collections.abc import Iterable

from az.tokenization import Token


def tokens_to_string(tokens: Iterable[Token], /) -> str:
    return ''.join(str(token.content) for token in tokens)
