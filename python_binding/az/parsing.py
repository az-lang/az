from __future__ import annotations

import typing as _t

if _t.TYPE_CHECKING:
    from .tokenization import PositionedToken, SubstringPosition

    # classes

    class Expression:
        pass

    class AnnotatedIdentifier(Expression):
        identifier: Identifier
        annotation: Expression

    class Assignment(Expression):
        target: Expression
        value: Expression

    class BinaryOperation(Expression):
        left: Expression
        right: Expression
        operator: BinaryArithmeticOperator

    class Block(Expression):
        statements: list[Statement]
        expression: Expression | None

    class Call(Expression):
        callable: Expression
        arguments: list[Expression]

    class Comparison(Expression):
        left: Expression
        operator: ComparisonOperator
        right: Expression

    class Conditional(Expression):
        antecedent: Expression
        consequent: Expression
        alternative: Expression

    class Identifier(Expression):
        string: str
        position: SubstringPosition

    class MemberAccess(Expression):
        object: Expression
        member: Identifier

    class NumericLiteral(Expression):
        string: str
        type_: NumericLiteralType
        position: SubstringPosition

    class Tuple(Expression):
        elements: list[Expression]

    class UnaryOperation(Expression):
        operand: Expression
        operator: UnaryArithmeticOperator

    class Statement:
        pass

    class ExpressionStatement(Statement):
        expression: Expression

    # enumerations

    class BinaryArithmeticOperator:
        ADDITION: _t.ClassVar[_t.Self]
        DIVISION: _t.ClassVar[_t.Self]
        MULTIPLICATION: _t.ClassVar[_t.Self]
        SUBTRACTION: _t.ClassVar[_t.Self]

    class ComparisonOperator:
        EQUAL_TO: _t.ClassVar[_t.Self]
        GREATER_THAN: _t.ClassVar[_t.Self]
        GREATER_THAN_OR_EQUAL_TO: _t.ClassVar[_t.Self]
        LOWER_THAN: _t.ClassVar[_t.Self]
        LOWER_THAN_OR_EQUAL_TO: _t.ClassVar[_t.Self]
        NOT_EQUAL_TO: _t.ClassVar[_t.Self]

    class NumericLiteralType:
        F32: _t.ClassVar[_t.Self]
        F64: _t.ClassVar[_t.Self]
        I8: _t.ClassVar[_t.Self]
        I16: _t.ClassVar[_t.Self]
        I32: _t.ClassVar[_t.Self]
        I64: _t.ClassVar[_t.Self]

    class UnaryArithmeticOperator:
        NEGATION: _t.ClassVar[_t.Self]

    # exceptions

    class ParsingError(Exception):
        pass

    class MismatchedOpenParentheses(ParsingError):
        token: PositionedToken

    class MissingSemicolon(ParsingError):
        token: PositionedToken

    class OutOfTokens(ParsingError):
        pass

    class UnexpectedToken(ParsingError):
        token: PositionedToken

    # functions

    def parse_tokens(tokens: list[PositionedToken]) -> list[Statement]:
        ...

    del PositionedToken, SubstringPosition
else:
    from . import _az

    Expression = _az.Expression
    AnnotatedIdentifier = _az.AnnotatedIdentifier
    Assignment = _az.Assignment
    BinaryOperation = _az.BinaryOperation
    Block = _az.Block
    Call = _az.Call
    Comparison = _az.Comparison
    Conditional = _az.Conditional
    Identifier = _az.Identifier
    MemberAccess = _az.MemberAccess
    NumericLiteral = _az.NumericLiteral
    Tuple = _az.Tuple
    UnaryOperation = _az.UnaryOperation

    Statement = _az.Statement
    ExpressionStatement = _az.ExpressionStatement

    # enumerations
    BinaryArithmeticOperator = _az.BinaryArithmeticOperator
    ComparisonOperator = _az.ComparisonOperator
    NumericLiteralType = _az.NumericLiteralType
    UnaryArithmeticOperator = _az.UnaryArithmeticOperator

    # exceptions
    ParsingError = _az.ParsingError
    MismatchedOpenParentheses = _az.MismatchedOpenParentheses
    MissingSemicolon = _az.MissingSemicolon
    OutOfTokens = _az.OutOfTokens
    UnexpectedToken = _az.UnexpectedToken

    # functions
    parse_tokens = _az.parse_tokens

    del _az
