from az.tokenization import PositionedToken


def tokens_to_string(tokens: list[PositionedToken]) -> str:
    return ''.join(
        positioned_token.token.string for positioned_token in tokens
    )
