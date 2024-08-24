from collections.abc import Callable, Iterable, Sequence
from functools import singledispatch
from itertools import chain
from typing import TypeVar

from az.parsing import (
    AnnotatedIdentifier,
    Assignment,
    BidirectionalConditional,
    BinaryArithmeticOperation,
    BinaryComparison,
    Block,
    Call,
    Expression,
    ExpressionStatement,
    Filler,
    FunctionDefinition,
    FunctionType,
    Grouping,
    Identifier,
    MemberAccess,
    NumericLiteral,
    Return,
    Script,
    Statement,
    Tuple,
    UnaryArithmeticOperation,
    UnidirectionalConditional,
    WhileLoop,
)


def script_to_fillers(script: Script, /) -> list[Filler]:
    return (
        list(
            chain.from_iterable(map(_statement_to_fillers, script.statements))
        )
        + script.fillers
    )


@singledispatch
def are_structurally_equivalent_expressions(
    first: Expression, second: Expression, /
) -> bool:
    raise TypeError(type(first))


@are_structurally_equivalent_expressions.register(AnnotatedIdentifier)
def _are_structurally_equivalent_annotated_identifiers(
    first: AnnotatedIdentifier, second: Expression, /
) -> bool:
    return (
        isinstance(second, AnnotatedIdentifier)
        and _are_structurally_equal_identifiers(
            first.identifier, second.identifier
        )
        and are_structurally_equivalent_expressions(
            first.annotation, second.annotation
        )
    )


@are_structurally_equivalent_expressions.register(Assignment)
def _(first: Assignment, second: Expression, /) -> bool:
    return (
        isinstance(second, Assignment)
        and are_structurally_equivalent_expressions(
            first.target, second.target
        )
        and are_structurally_equivalent_expressions(first.value, second.value)
    )


@are_structurally_equivalent_expressions.register(BidirectionalConditional)
def _(first: BidirectionalConditional, second: Expression, /) -> bool:
    return (
        isinstance(second, BidirectionalConditional)
        and are_structurally_equivalent_expressions(
            first.antecedent, second.antecedent
        )
        and _are_structurally_equivalent_blocks(
            first.consequent, second.consequent
        )
        and are_structurally_equivalent_expressions(
            first.alternative, second.alternative
        )
    )


@are_structurally_equivalent_expressions.register(BinaryArithmeticOperation)
def _(first: BinaryArithmeticOperation, second: Expression, /) -> bool:
    return (
        isinstance(second, BinaryArithmeticOperation)
        and first.operator is second.operator
        and are_structurally_equivalent_expressions(first.left, second.left)
        and are_structurally_equivalent_expressions(first.right, second.right)
    )


@are_structurally_equivalent_expressions.register(BinaryComparison)
def _(first: BinaryComparison, second: Expression, /) -> bool:
    return (
        isinstance(second, BinaryComparison)
        and first.operator is second.operator
        and are_structurally_equivalent_expressions(first.left, second.left)
        and are_structurally_equivalent_expressions(first.right, second.right)
    )


@are_structurally_equivalent_expressions.register(Block)
def _are_structurally_equivalent_blocks(
    first: Block, second: Expression, /
) -> bool:
    return (
        isinstance(second, Block)
        and _are_structurally_equivalent_statement_lists(
            first.statements, second.statements
        )
        and (
            second.expression is None
            if first.expression is None
            else (
                second.expression is not None
                and are_structurally_equivalent_expressions(
                    first.expression, second.expression
                )
            )
        )
    )


@are_structurally_equivalent_expressions.register(Call)
def _(first: Call, second: Expression, /) -> bool:
    return (
        isinstance(second, Call)
        and are_structurally_equivalent_expressions(
            first.callable, second.callable
        )
        and _are_structurally_equivalent_expression_lists(
            first.arguments, second.arguments
        )
    )


@are_structurally_equivalent_expressions.register(FunctionDefinition)
def _(first: FunctionDefinition, second: Expression, /) -> bool:
    return (
        isinstance(second, FunctionDefinition)
        and _are_structurally_equivalent_annotated_identifier_lists(
            first.parameters, second.parameters
        )
        and are_structurally_equivalent_expressions(
            first.return_type, second.return_type
        )
        and _are_structurally_equivalent_blocks(first.body, second.body)
    )


@are_structurally_equivalent_expressions.register(FunctionType)
def _(first: FunctionType, second: Expression, /) -> bool:
    return (
        isinstance(second, FunctionType)
        and _are_structurally_equivalent_expression_lists(
            first.parameters, second.parameters
        )
        and are_structurally_equivalent_expressions(
            first.return_type, second.return_type
        )
    )


@are_structurally_equivalent_expressions.register(Grouping)
def _(first: Grouping, second: Expression, /) -> bool:
    return isinstance(
        second, Grouping
    ) and are_structurally_equivalent_expressions(
        first.expression, second.expression
    )


@are_structurally_equivalent_expressions.register(Identifier)
def _are_structurally_equal_identifiers(
    first: Identifier, second: Expression, /
) -> bool:
    return isinstance(second, Identifier) and first.string == second.string


@are_structurally_equivalent_expressions.register(MemberAccess)
def _(first: MemberAccess, second: Expression, /) -> bool:
    return (
        isinstance(second, MemberAccess)
        and are_structurally_equivalent_expressions(
            first.object, second.object
        )
        and _are_structurally_equal_identifiers(first.member, second.member)
    )


@are_structurally_equivalent_expressions.register(NumericLiteral)
def _(first: NumericLiteral, second: Expression, /) -> bool:
    return (
        isinstance(second, NumericLiteral)
        and first.value == second.value
        and first.type_ is second.type_
    )


@are_structurally_equivalent_expressions.register(Return)
def _(first: Return, second: Expression, /) -> bool:
    return isinstance(
        second, Return
    ) and are_structurally_equivalent_expressions(
        first.expression, second.expression
    )


@are_structurally_equivalent_expressions.register(Tuple)
def _(first: Tuple, second: Expression, /) -> bool:
    return isinstance(
        second, Tuple
    ) and _are_structurally_equivalent_expression_lists(
        first.elements, second.elements
    )


@are_structurally_equivalent_expressions.register(UnaryArithmeticOperation)
def _(first: UnaryArithmeticOperation, second: Expression, /) -> bool:
    return (
        isinstance(second, UnaryArithmeticOperation)
        and first.operator is second.operator
        and are_structurally_equivalent_expressions(
            first.operand, second.operand
        )
    )


@are_structurally_equivalent_expressions.register(UnidirectionalConditional)
def _(first: UnidirectionalConditional, second: Expression, /) -> bool:
    return (
        isinstance(second, UnidirectionalConditional)
        and are_structurally_equivalent_expressions(
            first.antecedent, second.antecedent
        )
        and _are_structurally_equivalent_blocks(
            first.consequent, second.consequent
        )
    )


@are_structurally_equivalent_expressions.register(WhileLoop)
def _(first: WhileLoop, second: Expression, /) -> bool:
    return (
        isinstance(second, WhileLoop)
        and are_structurally_equivalent_expressions(
            first.condition, second.condition
        )
        and _are_structurally_equivalent_blocks(first.body, second.body)
    )


assert all(
    (
        are_structurally_equivalent_expressions.dispatch(cls)
        is not are_structurally_equivalent_expressions.dispatch(Expression)
    )
    for cls in Expression.__subclasses__()
), [
    cls
    for cls in Expression.__subclasses__()
    if (
        are_structurally_equivalent_expressions.dispatch(cls)
        is are_structurally_equivalent_expressions.dispatch(Expression)
    )
]


def _are_structurally_equivalent_expression_lists(
    first: list[Expression], second: list[Expression], /
) -> bool:
    return _are_equivalent_sequences(
        first, second, are_structurally_equivalent_expressions
    )


def _are_structurally_equivalent_annotated_identifier_lists(
    first: list[AnnotatedIdentifier], second: list[AnnotatedIdentifier], /
) -> bool:
    return _are_equivalent_sequences(
        first, second, _are_structurally_equivalent_annotated_identifiers
    )


_T = TypeVar('_T')


def _are_equivalent_sequences(
    first: list[_T],
    second: list[_T],
    elements_equivalence_relation: Callable[[_T, _T], bool],
    /,
) -> bool:
    return len(first) == len(second) and all(
        map(elements_equivalence_relation, first, second)
    )


def _are_structurally_equivalent_statement_lists(
    first: list[Statement], second: list[Statement], /
) -> bool:
    return _are_equivalent_sequences(
        first, second, _are_structurally_equivalent_statements
    )


@singledispatch
def _are_structurally_equivalent_statements(
    first: Statement, second: Statement, /
) -> bool:
    raise TypeError(type(first))


@_are_structurally_equivalent_statements.register(ExpressionStatement)
def _(first: ExpressionStatement, second: Statement, /) -> bool:
    return isinstance(
        second, ExpressionStatement
    ) and are_structurally_equivalent_expressions(
        first.expression, second.expression
    )


def _comma_separated_values_to_fillers(
    values: Sequence[Expression], comma_fillers: list[list[Filler]], /
) -> Iterable[Filler]:
    return (
        chain.from_iterable(
            _expression_to_fillers(value) + comma_fillers
            for value, comma_fillers in zip(values, comma_fillers, strict=True)
        )
        if len(values) == len(comma_fillers)
        else chain(
            chain.from_iterable(
                _expression_to_fillers(value) + comma_fillers
                for value, comma_fillers in zip(
                    values[:-1], comma_fillers, strict=True
                )
            ),
            _expression_to_fillers(values[-1]),
        )
    )


@singledispatch
def _expression_to_fillers(expression: Expression, /) -> list[Filler]:
    raise TypeError(type(expression))


@_expression_to_fillers.register(AnnotatedIdentifier)
def _(expression: AnnotatedIdentifier, /) -> list[Filler]:
    return list(
        chain(
            _expression_to_fillers(expression.identifier),
            expression.operator_fillers,
            _expression_to_fillers(expression.annotation),
        )
    )


@_expression_to_fillers.register(Assignment)
def _(expression: Assignment, /) -> list[Filler]:
    return list(
        chain(
            _expression_to_fillers(expression.target),
            expression.operator_fillers,
            _expression_to_fillers(expression.value),
        )
    )


@_expression_to_fillers.register(BidirectionalConditional)
def _(expression: BidirectionalConditional, /) -> list[Filler]:
    return list(
        chain(
            expression.antecedent_opener_fillers,
            _expression_to_fillers(expression.antecedent),
            _block_to_fillers(expression.consequent),
            expression.alternative_opener_fillers,
            _expression_to_fillers(expression.alternative),
        )
    )


@_expression_to_fillers.register(BinaryArithmeticOperation)
def _(expression: BinaryArithmeticOperation, /) -> list[Filler]:
    return list(
        chain(
            _expression_to_fillers(expression.left),
            expression.operator_fillers,
            _expression_to_fillers(expression.right),
        )
    )


@_expression_to_fillers.register(BinaryComparison)
def _(expression: BinaryComparison, /) -> list[Filler]:
    return list(
        chain(
            _expression_to_fillers(expression.left),
            expression.operator_fillers,
            _expression_to_fillers(expression.right),
        )
    )


@_expression_to_fillers.register(Block)
def _block_to_fillers(expression: Block, /) -> list[Filler]:
    return list(
        chain(
            expression.open_brace_fillers,
            chain.from_iterable(
                _statement_to_fillers(statement)
                for statement in expression.statements
            ),
            (
                []
                if expression.expression is None
                else _expression_to_fillers(expression.expression)
            ),
            expression.close_brace_fillers,
        )
    )


@_expression_to_fillers.register(Call)
def _(expression: Call, /) -> list[Filler]:
    return list(
        chain(
            _expression_to_fillers(expression.callable),
            expression.open_parenthesis_fillers,
            _comma_separated_values_to_fillers(
                expression.arguments, expression.comma_fillers
            ),
            expression.close_parenthesis_fillers,
        )
    )


@_expression_to_fillers.register(FunctionDefinition)
def _(expression: FunctionDefinition, /) -> list[Filler]:
    return list(
        chain(
            expression.opener_fillers,
            expression.open_parenthesis_fillers,
            _comma_separated_values_to_fillers(
                expression.parameters, expression.comma_fillers
            ),
            expression.close_parenthesis_fillers,
            expression.arrow_fillers,
            _expression_to_fillers(expression.return_type),
            _block_to_fillers(expression.body),
        )
    )


@_expression_to_fillers.register(FunctionType)
def _(expression: FunctionType, /) -> list[Filler]:
    return list(
        chain(
            expression.open_parenthesis_fillers,
            _comma_separated_values_to_fillers(
                expression.parameters, expression.comma_fillers
            ),
            expression.close_parenthesis_fillers,
            expression.operator_fillers,
            _expression_to_fillers(expression.return_type),
        )
    )


@_expression_to_fillers.register(Grouping)
def _(expression: Grouping, /) -> list[Filler]:
    return list(
        chain(
            expression.open_parenthesis_fillers,
            _expression_to_fillers(expression.expression),
            expression.close_parenthesis_fillers,
        )
    )


@_expression_to_fillers.register(Identifier)
def _(expression: Identifier, /) -> list[Filler]:
    return expression.fillers


@_expression_to_fillers.register(MemberAccess)
def _(expression: MemberAccess, /) -> list[Filler]:
    return list(
        chain(
            _expression_to_fillers(expression.object),
            expression.operator_fillers,
            _expression_to_fillers(expression.member),
        )
    )


@_expression_to_fillers.register(NumericLiteral)
def _(expression: NumericLiteral, /) -> list[Filler]:
    return expression.fillers


@_expression_to_fillers.register(Return)
def _(expression: Return, /) -> list[Filler]:
    return expression.operator_fillers + _expression_to_fillers(
        expression.expression
    )


@_expression_to_fillers.register(Tuple)
def _(expression: Tuple, /) -> list[Filler]:
    return list(
        chain(
            expression.open_parenthesis_fillers,
            _comma_separated_values_to_fillers(
                expression.elements, expression.comma_fillers
            ),
            expression.close_parenthesis_fillers,
        )
    )


@_expression_to_fillers.register(UnaryArithmeticOperation)
def _(expression: UnaryArithmeticOperation, /) -> list[Filler]:
    return expression.operator_fillers + _expression_to_fillers(
        expression.operand
    )


@_expression_to_fillers.register(UnidirectionalConditional)
def _(expression: UnidirectionalConditional, /) -> list[Filler]:
    return list(
        chain(
            expression.opener_fillers,
            _expression_to_fillers(expression.antecedent),
            _block_to_fillers(expression.consequent),
        )
    )


@_expression_to_fillers.register(WhileLoop)
def _(expression: WhileLoop, /) -> list[Filler]:
    return list(
        chain(
            expression.opener_fillers,
            _expression_to_fillers(expression.condition),
            _block_to_fillers(expression.body),
        )
    )


assert all(
    (
        _expression_to_fillers.dispatch(cls)
        is not _expression_to_fillers.dispatch(Expression)
    )
    for cls in Expression.__subclasses__()
), [
    cls
    for cls in Expression.__subclasses__()
    if (
        _expression_to_fillers.dispatch(cls)
        is _expression_to_fillers.dispatch(Expression)
    )
]


@singledispatch
def _statement_to_fillers(statement: Statement, /) -> list[Filler]:
    raise TypeError(type(statement))


@_statement_to_fillers.register(ExpressionStatement)
def _(statement: ExpressionStatement, /) -> list[Filler]:
    return (
        _expression_to_fillers(statement.expression)
        + statement.semicolon_fillers
    )
