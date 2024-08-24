from __future__ import annotations

from abc import ABC as _ABC, abstractmethod as _abstractmethod
from typing import (
    Any as _Any,
    ClassVar as _ClassVar,
    Self as _Self,
    TYPE_CHECKING,
    final as _final,
    overload as _overload,
)

from .tokenization import (
    SubstringPosition as _SubstringPosition,
    Token as _Token,
    TokenCollection as _TokenCollection,
)

if TYPE_CHECKING:
    # classes

    @_final
    class Precedence:
        MIN: _ClassVar[int]

        @_overload
        def __eq__(self, other: _Self) -> bool: ...

        @_overload
        def __eq__(self, other: _Any) -> _Any: ...

        def __eq__(self, other: _Any) -> _Any: ...

        @_overload
        def __ge__(self, other: _Self) -> bool: ...

        @_overload
        def __ge__(self, other: _Any) -> _Any: ...

        def __ge__(self, other: _Any) -> _Any: ...

        @_overload
        def __gt__(self, other: _Self) -> bool: ...

        @_overload
        def __gt__(self, other: _Any) -> _Any: ...

        def __gt__(self, other: _Any) -> _Any: ...

        @_overload
        def __le__(self, other: _Self) -> bool: ...

        @_overload
        def __le__(self, other: _Any) -> _Any: ...

        def __le__(self, other: _Any) -> _Any: ...

        @_overload
        def __lt__(self, other: _Self) -> bool: ...

        @_overload
        def __lt__(self, other: _Any) -> _Any: ...

        def __lt__(self, other: _Any) -> _Any: ...

    @_final
    class BinaryAdditionOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinaryDivisionOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinaryMultiplicationOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinarySubtractionOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    BinaryArithmeticOperator = (
        type[BinaryAdditionOperator]
        | type[BinaryDivisionOperator]
        | type[BinaryMultiplicationOperator]
        | type[BinarySubtractionOperator]
    )

    @_final
    class AnnotationOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class AssignmentOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class CallOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class FunctionTypeOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class MemberAccessOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinaryEqualToOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinaryGreaterThanOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinaryGreaterThanOrEqualToOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinaryLessThanOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinaryLessThanOrEqualToOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class BinaryNotEqualToOperator:
        ASSOCIATIVITY: _ClassVar[Associativity]
        PRECEDENCE: _ClassVar[Precedence]

    BinaryComparisonOperator = (
        type[BinaryEqualToOperator]
        | type[BinaryGreaterThanOperator]
        | type[BinaryGreaterThanOrEqualToOperator]
        | type[BinaryLessThanOperator]
        | type[BinaryLessThanOrEqualToOperator]
        | type[BinaryNotEqualToOperator]
    )

    @_final
    class ReturnOperator:
        PRECEDENCE: _ClassVar[Precedence]

    @_final
    class UnaryNegationOperator:
        PRECEDENCE: _ClassVar[Precedence]

    UnaryArithmeticOperator = type[UnaryNegationOperator]

    @_final
    class Filler:
        content: FillerContent
        position: _SubstringPosition

        def validate_contents(self, /) -> None: ...

        def validate_positions(self, /) -> None: ...

        def __init__(
            self, content: FillerContent, /, *, position: _SubstringPosition
        ) -> None: ...

    @_final
    class FillerContent:
        kind: FillerKind
        state: str | None

        def validate(self, /) -> None: ...

        def __init__(
            self, kind: FillerKind, state: str | None = None, /
        ) -> None: ...

    class Expression(_ABC):
        @_abstractmethod
        def __init__(self) -> None: ...

    @_final
    class AnnotatedIdentifier(Expression):
        identifier: Identifier
        annotation: Expression
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            identifier: Identifier,
            annotation: Expression,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_final
    class Assignment(Expression):
        target: Expression
        value: Expression
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            target: Expression,
            value: Expression,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_final
    class BidirectionalConditional(Expression):
        antecedent: Expression
        consequent: Block
        alternative: Expression
        antecedent_opener_position: _SubstringPosition
        alternative_opener_position: _SubstringPosition
        antecedent_opener_fillers: list[Filler]
        alternative_opener_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            antecedent: Expression,
            consequent: Block,
            alternative: Expression,
            /,
            *,
            antecedent_opener_position: _SubstringPosition,
            alternative_opener_position: _SubstringPosition,
            antecedent_opener_fillers: list[Filler],
            alternative_opener_fillers: list[Filler],
        ) -> None: ...

    @_final
    class BinaryArithmeticOperation(Expression):
        left: Expression
        right: Expression
        operator: BinaryArithmeticOperator
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            left: Expression,
            right: Expression,
            operator: BinaryArithmeticOperator,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_final
    class BinaryComparison(Expression):
        left: Expression
        right: Expression
        operator: BinaryComparisonOperator
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            left: Expression,
            right: Expression,
            operator: BinaryComparisonOperator,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_final
    class Block(Expression):
        statements: list[Statement]
        expression: Expression | None
        open_brace_position: _SubstringPosition
        close_brace_position: _SubstringPosition
        open_brace_fillers: list[Filler]
        close_brace_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            statements: list[Statement],
            expression: Expression | None,
            /,
            *,
            open_brace_position: _SubstringPosition,
            close_brace_position: _SubstringPosition,
            open_brace_fillers: list[Filler],
            close_brace_fillers: list[Filler],
        ) -> None: ...

    @_final
    class Call(Expression):
        callable: Expression
        arguments: list[Expression]
        open_parenthesis_position: _SubstringPosition
        comma_positions: list[_SubstringPosition]
        close_parenthesis_position: _SubstringPosition
        open_parenthesis_fillers: list[Filler]
        comma_fillers: list[list[Filler]]
        close_parenthesis_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            callable_: Expression,
            arguments: list[Expression],
            /,
            *,
            open_parenthesis_position: _SubstringPosition,
            comma_positions: list[_SubstringPosition],
            close_parenthesis_position: _SubstringPosition,
            open_parenthesis_fillers: list[Filler],
            comma_fillers: list[list[Filler]],
            close_parenthesis_fillers: list[Filler],
        ) -> None: ...

    @_final
    class FunctionDefinition(Expression):
        parameters: list[AnnotatedIdentifier]
        return_type: Expression
        body: Block
        opener_position: _SubstringPosition
        open_parenthesis_position: _SubstringPosition
        comma_positions: list[_SubstringPosition]
        close_parenthesis_position: _SubstringPosition
        arrow_position: _SubstringPosition
        opener_fillers: list[Filler]
        open_parenthesis_fillers: list[Filler]
        comma_fillers: list[list[Filler]]
        close_parenthesis_fillers: list[Filler]
        arrow_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            parameters: list[AnnotatedIdentifier],
            return_type: Expression,
            body: Block,
            /,
            *,
            opener_position: _SubstringPosition,
            open_parenthesis_position: _SubstringPosition,
            comma_positions: list[_SubstringPosition],
            close_parenthesis_position: _SubstringPosition,
            arrow_position: _SubstringPosition,
            opener_fillers: list[Filler],
            open_parenthesis_fillers: list[Filler],
            comma_fillers: list[list[Filler]],
            close_parenthesis_fillers: list[Filler],
            arrow_fillers: list[Filler],
        ) -> None: ...

    @_final
    class FunctionType(Expression):
        parameters: list[Expression]
        return_type: Expression
        open_parenthesis_position: _SubstringPosition
        comma_positions: list[_SubstringPosition]
        close_parenthesis_position: _SubstringPosition
        operator_position: _SubstringPosition
        open_parenthesis_fillers: list[Filler]
        comma_fillers: list[list[Filler]]
        close_parenthesis_fillers: list[Filler]
        operator_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            parameters: list[Expression],
            return_type: Expression,
            /,
            *,
            open_parenthesis_position: _SubstringPosition,
            comma_positions: list[_SubstringPosition],
            close_parenthesis_position: _SubstringPosition,
            operator_position: _SubstringPosition,
            open_parenthesis_fillers: list[Filler],
            comma_fillers: list[list[Filler]],
            close_parenthesis_fillers: list[Filler],
            operator_fillers: list[Filler],
        ) -> None: ...

    @_final
    class Grouping(Expression):
        expression: Expression
        open_parenthesis_position: _SubstringPosition
        close_parenthesis_position: _SubstringPosition
        open_parenthesis_fillers: list[Filler]
        close_parenthesis_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            expression: Expression,
            /,
            *,
            open_parenthesis_position: _SubstringPosition,
            close_parenthesis_position: _SubstringPosition,
            open_parenthesis_fillers: list[Filler],
            close_parenthesis_fillers: list[Filler],
        ) -> None: ...

    @_final
    class Identifier(Expression):
        string: str
        position: _SubstringPosition
        fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            string: str,
            /,
            *,
            position: _SubstringPosition,
            fillers: list[Filler],
        ) -> None: ...

    @_final
    class MemberAccess(Expression):
        object: Expression
        member: Identifier
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            object_: Expression,
            member: Identifier,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_final
    class NumericLiteral(Expression):
        value: str
        type_: NumericLiteralType
        position: _SubstringPosition
        fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            value: str,
            type_: NumericLiteralType,
            /,
            *,
            position: _SubstringPosition,
            fillers: list[Filler],
        ) -> None: ...

    @_final
    class Return(Expression):
        expression: Expression
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            expression: Expression,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_final
    class Tuple(Expression):
        elements: list[Expression]
        open_parenthesis_position: _SubstringPosition
        comma_positions: list[_SubstringPosition]
        close_parenthesis_position: _SubstringPosition
        open_parenthesis_fillers: list[Filler]
        comma_fillers: list[list[Filler]]
        close_parenthesis_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            elements: list[Expression],
            /,
            *,
            open_parenthesis_position: _SubstringPosition,
            comma_positions: list[_SubstringPosition],
            close_parenthesis_position: _SubstringPosition,
            open_parenthesis_fillers: list[Filler],
            comma_fillers: list[list[Filler]],
            close_parenthesis_fillers: list[Filler],
        ) -> None: ...

    @_final
    class UnaryArithmeticOperation(Expression):
        operand: Expression
        operator: UnaryArithmeticOperator
        operator_position: _SubstringPosition
        operator_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            operand: Expression,
            operator: UnaryArithmeticOperator,
            /,
            *,
            operator_position: _SubstringPosition,
            operator_fillers: list[Filler],
        ) -> None: ...

    @_final
    class UnidirectionalConditional(Expression):
        antecedent: Expression
        consequent: Block
        opener_position: _SubstringPosition
        opener_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            antecedent: Expression,
            consequent: Block,
            /,
            *,
            opener_position: _SubstringPosition,
            opener_fillers: list[Filler],
        ) -> None: ...

    @_final
    class WhileLoop(Expression):
        condition: Expression
        body: Block
        opener_position: _SubstringPosition
        opener_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            condition: Expression,
            body: Block,
            /,
            *,
            opener_position: _SubstringPosition,
            opener_fillers: list[Filler],
        ) -> None: ...

    @_final
    class Script:
        statements: list[Statement]
        fillers: list[Filler]

        @classmethod
        def from_tokens(cls, tokens: _TokenCollection, /) -> _Self: ...

        def format(self, /) -> None: ...

        def reset_positions(self, /) -> None: ...

        def tokenize(self, /) -> _TokenCollection: ...

        def validate_contents(self, /) -> None: ...

        def validate_positions(self, /) -> None: ...

        def __init__(
            self, statements: list[Statement], /, *, fillers: list[Filler]
        ) -> None: ...

    class Statement(_ABC):
        @_abstractmethod
        def __init__(self) -> None: ...

    @_final
    class ExpressionStatement(Statement):
        expression: Expression
        semicolon_position: _SubstringPosition
        semicolon_fillers: list[Filler]

        def validate_contents(self, /) -> None: ...

        def __init__(
            self,
            expression: Expression,
            /,
            *,
            semicolon_position: _SubstringPosition,
            semicolon_fillers: list[Filler],
        ) -> None: ...

    # enumerations

    @_final
    class Associativity:
        LEFT_TO_RIGHT: _ClassVar[_Self]
        RIGHT_TO_LEFT: _ClassVar[_Self]

    @_final
    class FillerKind:
        COMMENT_BLOCK: _ClassVar[_Self]
        COMMENT_LINE: _ClassVar[_Self]
        NEWLINE: _ClassVar[_Self]
        WHITESPACE: _ClassVar[_Self]

    @_final
    class NumericLiteralType:
        F32: _ClassVar[_Self]
        F64: _ClassVar[_Self]
        I8: _ClassVar[_Self]
        I16: _ClassVar[_Self]
        I32: _ClassVar[_Self]
        I64: _ClassVar[_Self]
        ISIZE: _ClassVar[_Self]
        U8: _ClassVar[_Self]
        U16: _ClassVar[_Self]
        U32: _ClassVar[_Self]
        U64: _ClassVar[_Self]
        USIZE: _ClassVar[_Self]

    # exceptions

    class ParsingError(Exception):
        pass

    @_final
    class MismatchedOpenBrace(ParsingError):
        position: _SubstringPosition

        def __init__(self, position: _SubstringPosition, /) -> None: ...

    @_final
    class MismatchedOpenParenthesis(ParsingError):
        position: _SubstringPosition

        def __init__(self, position: _SubstringPosition, /) -> None: ...

    @_final
    class MissingSemicolon(ParsingError):
        token: _Token

        def __init__(self, token: _Token, /) -> None: ...

    @_final
    class OutOfTokens(ParsingError):
        def __init__(self, /) -> None: ...

    @_final
    class UnexpectedExpression(ParsingError):
        expression: Expression

        def __init__(self, expression: Expression, /) -> None: ...

    @_final
    class UnexpectedToken(ParsingError):
        token: _Token

        def __init__(self, token: _Token, /) -> None: ...

    def is_keyword(value: str, /) -> bool: ...

else:
    from . import _az

    # classes
    Filler = _az.Filler
    FillerContent = _az.FillerContent
    Precedence = _az.Precedence

    AnnotationOperator = _az.AnnotationOperator
    AssignmentOperator = _az.AssignmentOperator

    BinaryAdditionOperator = _az.BinaryAdditionOperator
    BinaryDivisionOperator = _az.BinaryDivisionOperator
    BinaryMultiplicationOperator = _az.BinaryMultiplicationOperator
    BinarySubtractionOperator = _az.BinarySubtractionOperator

    BinaryArithmeticOperator = (
        type[BinaryAdditionOperator]
        | type[BinaryDivisionOperator]
        | type[BinaryMultiplicationOperator]
        | type[BinarySubtractionOperator]
    )

    BinaryEqualToOperator = _az.BinaryEqualToOperator
    BinaryGreaterThanOperator = _az.BinaryGreaterThanOperator
    BinaryGreaterThanOrEqualToOperator = _az.BinaryGreaterThanOrEqualToOperator
    BinaryLessThanOperator = _az.BinaryLessThanOperator
    BinaryLessThanOrEqualToOperator = _az.BinaryLessThanOrEqualToOperator
    BinaryNotEqualToOperator = _az.BinaryNotEqualToOperator

    BinaryComparisonOperator = (
        type[BinaryEqualToOperator]
        | type[BinaryGreaterThanOperator]
        | type[BinaryGreaterThanOrEqualToOperator]
        | type[BinaryLessThanOperator]
        | type[BinaryLessThanOrEqualToOperator]
        | type[BinaryNotEqualToOperator]
    )

    CallOperator = _az.CallOperator
    FunctionTypeOperator = _az.FunctionTypeOperator
    MemberAccessOperator = _az.MemberAccessOperator

    ReturnOperator = _az.ReturnOperator
    UnaryNegationOperator = _az.UnaryNegationOperator

    UnaryArithmeticOperator = type[UnaryNegationOperator]

    Expression = _az.Expression
    AnnotatedIdentifier = _az.AnnotatedIdentifier
    Assignment = _az.Assignment
    BidirectionalConditional = _az.BidirectionalConditional
    BinaryArithmeticOperation = _az.BinaryArithmeticOperation
    BinaryComparison = _az.BinaryComparison
    Block = _az.Block
    Call = _az.Call
    FunctionDefinition = _az.FunctionDefinition
    FunctionType = _az.FunctionType
    Grouping = _az.Grouping
    Identifier = _az.Identifier
    MemberAccess = _az.MemberAccess
    NumericLiteral = _az.NumericLiteral
    Return = _az.Return
    Tuple = _az.Tuple
    UnaryArithmeticOperation = _az.UnaryArithmeticOperation
    UnidirectionalConditional = _az.UnidirectionalConditional
    WhileLoop = _az.WhileLoop

    Script = _az.Script

    Statement = _az.Statement
    ExpressionStatement = _az.ExpressionStatement

    # enumerations
    Associativity = _az.Associativity
    FillerKind = _az.FillerKind
    NumericLiteralType = _az.NumericLiteralType

    # exceptions
    ParsingError = _az.ParsingError
    MismatchedOpenBrace = _az.MismatchedOpenBrace
    MismatchedOpenParenthesis = _az.MismatchedOpenParenthesis
    MissingSemicolon = _az.MissingSemicolon
    OutOfTokens = _az.OutOfTokens
    UnexpectedExpression = _az.UnexpectedExpression
    UnexpectedToken = _az.UnexpectedToken

    # functions
    is_keyword = _az.is_keyword

    del _az
del TYPE_CHECKING
