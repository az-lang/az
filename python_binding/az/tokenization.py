from __future__ import annotations

from collections.abc import Iterator as _Iterator
from typing import (
    Any as _Any,
    ClassVar as _ClassVar,
    Self as _Self,
    TYPE_CHECKING,
    final as _final,
    overload as _overload,
)

if TYPE_CHECKING:
    # classes

    @_final
    class ByteCount:
        MAX: _ClassVar[_Self]
        MIN: _ClassVar[_Self]

        def __init__(self, _value: int | None = None, /) -> None: ...

        @_overload
        def __add__(self, other: _Self) -> _Self: ...

        @_overload
        def __add__(self, other: _Any) -> _Any: ...

        def __add__(self, other: _Any) -> _Any: ...

        @_overload
        def __eq__(self, other: _Self) -> bool: ...

        @_overload
        def __eq__(self, other: _Any) -> _Any: ...

        def __eq__(self, other: _Any) -> _Any: ...

        def __ge__(self, other: _Self) -> bool: ...

        def __gt__(self, other: _Self) -> bool: ...

        def __int__(self) -> int: ...

        def __le__(self, other: _Self) -> bool: ...

        def __lt__(self, other: _Self) -> bool: ...

        @_overload
        def __sub__(self, other: _Self) -> _Self: ...

        @_overload
        def __sub__(self, other: _Any) -> _Any: ...

        def __sub__(self, other: _Any) -> _Any: ...

    @_final
    class Utf8Count:
        MAX: _ClassVar[_Self]
        MIN: _ClassVar[_Self]

        def __init__(self, _value: int | None = None, /) -> None: ...

        @_overload
        def __add__(self, other: _Self) -> _Self: ...

        @_overload
        def __add__(self, other: _Any) -> _Any: ...

        def __add__(self, other: _Any) -> _Any: ...

        @_overload
        def __eq__(self, other: _Self) -> bool: ...

        @_overload
        def __eq__(self, other: _Any) -> _Any: ...

        def __eq__(self, other: _Any) -> _Any: ...

        def __ge__(self, other: _Self) -> bool: ...

        def __gt__(self, other: _Self) -> bool: ...

        def __int__(self) -> int: ...

        def __le__(self, other: _Self) -> bool: ...

        def __lt__(self, other: _Self) -> bool: ...

        @_overload
        def __sub__(self, other: _Self) -> _Self: ...

        @_overload
        def __sub__(self, other: _Any) -> _Any: ...

        def __sub__(self, other: _Any) -> _Any: ...

    @_final
    class CharacterPosition:
        byte: ByteCount
        utf_8: Utf8Count

        def __init__(self, *, byte: ByteCount, utf_8: Utf8Count) -> None: ...

    @_final
    class SubstringPosition:
        start: CharacterPosition
        end: CharacterPosition

        def __init__(
            self, *, start: CharacterPosition, end: CharacterPosition
        ) -> None: ...

    @_final
    class Token:
        content: TokenContent
        position: SubstringPosition

        def validate_contents(self, /) -> None: ...

        def validate_positions(self, /) -> None: ...

        def __init__(
            self, content: TokenContent, /, *, position: SubstringPosition
        ) -> None: ...

    @_final
    class TokenCollection:
        @classmethod
        def from_string(cls, string: str, /) -> _Self: ...

        def validate_contents(self, /) -> None: ...

        def validate_positions(self, /) -> None: ...

        def __init__(self, _tokens: list[Token], /) -> None: ...

        def __iter__(self, /) -> _Iterator[Token]: ...

    @_final
    class TokenContent:
        kind: TokenKind
        state: str | None

        def validate(self, /) -> None: ...

        def __init__(
            self, kind: TokenKind, state: str | None = None, /
        ) -> None: ...

    # enumerations

    @_final
    class NumericLiteralValueKind:
        FLOATING_POINT: _ClassVar[_Self]
        INTEGER: _ClassVar[_Self]

    @_final
    class TokenKind:
        ARROW: _ClassVar[_Self]
        ASSIGNMENT: _ClassVar[_Self]
        ASTERISK: _ClassVar[_Self]
        CLOSE_BRACE: _ClassVar[_Self]
        CLOSE_PARENTHESIS: _ClassVar[_Self]
        COLON: _ClassVar[_Self]
        COMMA: _ClassVar[_Self]
        COMMENT_BLOCK: _ClassVar[_Self]
        COMMENT_LINE: _ClassVar[_Self]
        DOT: _ClassVar[_Self]
        EQUAL_TO: _ClassVar[_Self]
        F32: _ClassVar[_Self]
        F64: _ClassVar[_Self]
        GREATER_THAN: _ClassVar[_Self]
        GREATER_THAN_OR_EQUAL_TO: _ClassVar[_Self]
        I8: _ClassVar[_Self]
        I16: _ClassVar[_Self]
        I32: _ClassVar[_Self]
        I64: _ClassVar[_Self]
        IDENTIFIER: _ClassVar[_Self]
        ISIZE: _ClassVar[_Self]
        LESS_THAN: _ClassVar[_Self]
        LESS_THAN_OR_EQUAL_TO: _ClassVar[_Self]
        MINUS: _ClassVar[_Self]
        NEWLINE: _ClassVar[_Self]
        NOT_EQUAL_TO: _ClassVar[_Self]
        OPEN_BRACE: _ClassVar[_Self]
        OPEN_PARENTHESIS: _ClassVar[_Self]
        PLUS: _ClassVar[_Self]
        SEMICOLON: _ClassVar[_Self]
        SLASH: _ClassVar[_Self]
        U8: _ClassVar[_Self]
        U16: _ClassVar[_Self]
        U32: _ClassVar[_Self]
        U64: _ClassVar[_Self]
        USIZE: _ClassVar[_Self]
        WHITESPACE: _ClassVar[_Self]

    # exceptions

    class LexicalError(Exception):
        pass

    @_final
    class CommentBlockIncomplete(LexicalError):
        position: SubstringPosition
        string: str

        def __init__(
            self, position: SubstringPosition, string: str, /
        ) -> None: ...

    @_final
    class IdentifierIncomplete(LexicalError):
        position: SubstringPosition
        string: str

        def __init__(
            self, position: SubstringPosition, string: str, /
        ) -> None: ...

    @_final
    class IdentifierUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        position: SubstringPosition
        string: str

        def __init__(
            self,
            character: str,
            expected: str,
            position: SubstringPosition,
            string: str,
            /,
        ) -> None: ...

    @_final
    class NumericLiteralTypeSuffixIncomplete(LexicalError):
        position: SubstringPosition
        string: str
        value: str
        value_kind: NumericLiteralValueKind

        def __init__(
            self,
            position: SubstringPosition,
            string: str,
            value: str,
            value_kind: NumericLiteralValueKind,
            /,
        ) -> None: ...

    @_final
    class NumericLiteralTypeSuffixUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        position: SubstringPosition
        string: str
        value: str
        value_kind: NumericLiteralValueKind

        def __init__(
            self,
            character: str,
            expected: str,
            position: SubstringPosition,
            string: str,
            value: str,
            value_kind: NumericLiteralValueKind,
            /,
        ) -> None: ...

    @_final
    class NumericLiteralTypeSuffixUnknown(LexicalError):
        position: SubstringPosition
        string: str
        type_suffix: str
        value: str
        value_kind: NumericLiteralValueKind

        def __init__(
            self,
            position: SubstringPosition,
            string: str,
            type_suffix: str,
            value: str,
            value_kind: NumericLiteralValueKind,
            /,
        ) -> None: ...

    @_final
    class NumericLiteralValueIncomplete(LexicalError):
        kind: NumericLiteralValueKind
        position: SubstringPosition
        string: str

        def __init__(
            self,
            kind: NumericLiteralValueKind,
            position: SubstringPosition,
            string: str,
            /,
        ) -> None: ...

    @_final
    class NumericLiteralValueTypeSuffixConflict(LexicalError):
        position: SubstringPosition
        string: str
        type_suffix: str
        value: str
        value_kind: NumericLiteralValueKind

        def __init__(
            self,
            position: SubstringPosition,
            string: str,
            type_suffix: str,
            value: str,
            value_kind: NumericLiteralValueKind,
            /,
        ) -> None: ...

    @_final
    class NumericLiteralValueUnexpectedCharacter(LexicalError):
        character: str
        expected: str
        kind: NumericLiteralValueKind
        position: SubstringPosition
        string: str

        def __init__(
            self,
            character: str,
            expected: str,
            kind: NumericLiteralValueKind,
            position: SubstringPosition,
            string: str,
            /,
        ) -> None: ...

    @_final
    class UnexpectedCharacter(LexicalError):
        character: str
        position: SubstringPosition
        string: str

        def __init__(
            self, character: str, position: SubstringPosition, string: str, /
        ) -> None: ...

    class ValidationError(Exception):
        pass

    @_final
    class ContentsValidationError(ValidationError):
        message: str

        def __init__(self, message: str, /) -> None: ...

    @_final
    class PositionsValidationError(ValidationError):
        message: str

        def __init__(self, message: str, /) -> None: ...

else:
    from . import _az

    # classes
    ByteCount = _az.ByteCount
    CharacterPosition = _az.CharacterPosition
    SubstringPosition = _az.SubstringPosition
    Token = _az.Token
    TokenCollection = _az.TokenCollection
    TokenContent = _az.TokenContent
    Utf8Count = _az.Utf8Count

    # enumerations
    NumericLiteralValueKind = _az.NumericLiteralValueKind
    TokenKind = _az.TokenKind

    # exceptions
    LexicalError = _az.LexicalError
    CommentBlockIncomplete = _az.CommentBlockIncomplete
    ContentsValidationError = _az.ContentsValidationError
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
    PositionsValidationError = _az.PositionsValidationError
    UnexpectedCharacter = _az.UnexpectedCharacter
    ValidationError = _az.ValidationError

    del _az
del TYPE_CHECKING
