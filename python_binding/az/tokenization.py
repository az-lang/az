from __future__ import annotations

import typing as _t

if _t.TYPE_CHECKING:
    # classes

    class CharacterPosition:
        byte: int
        utf_8: int

    class PositionedToken:
        position: SubstringPosition
        token: Token

    class SubstringPosition:
        start_line: int
        start_character_position: CharacterPosition
        end_line: int
        end_character_position: CharacterPosition

    class Token:
        kind: TokenKind
        string: str

    # enumerations

    class NumericLiteralValueKind:
        FLOATING_POINT: _t.ClassVar[_t.Self]
        INTEGER: _t.ClassVar[_t.Self]

    class TokenKind:
        ARROW: _t.ClassVar[_t.Self]
        ASSIGNMENT: _t.ClassVar[_t.Self]
        ASTERISK: _t.ClassVar[_t.Self]
        CLOSE_BRACE: _t.ClassVar[_t.Self]
        CLOSE_PARENTHESIS: _t.ClassVar[_t.Self]
        COLON: _t.ClassVar[_t.Self]
        COMMA: _t.ClassVar[_t.Self]
        COMMENT_BLOCK: _t.ClassVar[_t.Self]
        COMMENT_LINE: _t.ClassVar[_t.Self]
        DOT: _t.ClassVar[_t.Self]
        EQUAL_TO: _t.ClassVar[_t.Self]
        F32: _t.ClassVar[_t.Self]
        F64: _t.ClassVar[_t.Self]
        GREATER_THAN: _t.ClassVar[_t.Self]
        GREATER_THAN_OR_EQUAL_TO: _t.ClassVar[_t.Self]
        I8: _t.ClassVar[_t.Self]
        I16: _t.ClassVar[_t.Self]
        I32: _t.ClassVar[_t.Self]
        I64: _t.ClassVar[_t.Self]
        IDENTIFIER: _t.ClassVar[_t.Self]
        LOWER_THAN: _t.ClassVar[_t.Self]
        LOWER_THAN_OR_EQUAL_TO: _t.ClassVar[_t.Self]
        MINUS: _t.ClassVar[_t.Self]
        NEWLINE: _t.ClassVar[_t.Self]
        NOT_EQUAL_TO: _t.ClassVar[_t.Self]
        OPEN_BRACE: _t.ClassVar[_t.Self]
        OPEN_PARENTHESIS: _t.ClassVar[_t.Self]
        PLUS: _t.ClassVar[_t.Self]
        SEMICOLON: _t.ClassVar[_t.Self]
        SLASH: _t.ClassVar[_t.Self]
        U8: _t.ClassVar[_t.Self]
        U16: _t.ClassVar[_t.Self]
        U32: _t.ClassVar[_t.Self]
        U64: _t.ClassVar[_t.Self]
        WHITESPACE: _t.ClassVar[_t.Self]

    # exceptions

    class LexicalError(Exception):
        pass

    class CommentBlockIncomplete(LexicalError):
        position: SubstringPosition
        strings: list[str]

    class IdentifierIncomplete(LexicalError):
        position: SubstringPosition
        string: str

    class IdentifierUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        position: SubstringPosition
        string: str

    class NumericLiteralTypeSuffixIncomplete(LexicalError):
        position: SubstringPosition
        string: str
        value: str
        value_kind: NumericLiteralValueKind

    class NumericLiteralTypeSuffixUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        position: SubstringPosition
        string: str
        value: str
        value_kind: NumericLiteralValueKind

    class NumericLiteralTypeSuffixUnknown(LexicalError):
        position: SubstringPosition
        string: str
        type_suffix: str
        value: str
        value_kind: NumericLiteralValueKind

    class NumericLiteralValueIncomplete(LexicalError):
        kind: NumericLiteralValueKind
        position: SubstringPosition
        string: str

    class NumericLiteralValueTypeSuffixConflict(LexicalError):
        position: SubstringPosition
        type_suffix: str
        string: str
        value: str
        value_kind: NumericLiteralValueKind

    class NumericLiteralValueUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        kind: NumericLiteralValueKind
        position: SubstringPosition
        string: str

    class UnexpectedCharacter(LexicalError):
        character: str
        position: SubstringPosition
        string: str

    # functions

    def tokenize_string(string: str) -> list[PositionedToken]:
        ...

else:
    from . import _az

    # classes
    CharacterPosition = _az.CharacterPosition
    SubstringPosition = _az.SubstringPosition
    PositionedToken = _az.PositionedToken
    Token = _az.Token

    # enumerations
    NumericLiteralValueKind = _az.NumericLiteralValueKind
    TokenKind = _az.TokenKind

    # exceptions
    LexicalError = _az.LexicalError
    CommentBlockIncomplete = _az.CommentBlockIncomplete
    IdentifierIncomplete = _az.IdentifierIncomplete
    IdentifierUnexpectedCharacter = _az.IdentifierUnexpectedCharacter
    NumericLiteralTypeSuffixIncomplete = _az.NumericLiteralTypeSuffixIncomplete
    NumericLiteralTypeSuffixUnexpectedCharacter = (
        _az.NumericLiteralTypeSuffixUnexpectedCharacter
    )
    NumericLiteralTypeSuffixUnknown = _az.NumericLiteralTypeSuffixUnknown
    NumericLiteralValueIncomplete = _az.NumericLiteralValueIncomplete
    NumericLiteralValueTypeSuffixConflict = (
        _az.NumericLiteralValueTypeSuffixConflict
    )
    NumericLiteralValueUnexpectedCharacter = (
        _az.NumericLiteralValueUnexpectedCharacter
    )
    UnexpectedCharacter = _az.UnexpectedCharacter

    # functions
    tokenize_string = _az.tokenize_string

    del _az
