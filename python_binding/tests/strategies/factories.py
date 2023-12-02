import itertools
import sys
from functools import singledispatch

from hypothesis import strategies as _st

from az.parsing import (
    AnnotatedIdentifier,
    Assignment,
    BinaryArithmeticOperation,
    BinaryArithmeticOperator,
    BinaryComparison,
    BinaryComparisonOperator,
    BinaryDivisionOperator,
    Block,
    Call,
    Conditional,
    Expression,
    ExpressionStatement,
    Filler,
    FillerContent,
    FillerKind,
    FunctionDefinition,
    Grouping,
    Identifier,
    MemberAccess,
    NumericLiteral,
    Statement,
    Tuple,
    UnaryArithmeticOperation,
    UnaryArithmeticOperator,
)
from az.tokenization import ByteIndex as _ByteIndex
from az.tokenization import CharacterPosition as _CharacterPosition
from az.tokenization import SubstringPosition
from az.tokenization import SubstringPosition as _SubstringPosition
from az.tokenization import Utf8Index as _Utf8Index

MAX_FILLERS_SIZE = 4
MAX_EXPRESSIONS_SIZE = 4
MAX_STATEMENTS_SIZE = 4

byte_indices = _st.builds(_ByteIndex, _st.integers(0, int(_ByteIndex.MAX)))
utf8_indices = _st.builds(_Utf8Index, _st.integers(0, int(_Utf8Index.MAX)))
characters_positions = _st.builds(
    _CharacterPosition, byte=byte_indices, utf_8=utf8_indices
)
lines_indices = _st.integers(0, sys.maxsize)
substrings_positions = _st.builds(
    _SubstringPosition,
    start_line=lines_indices,
    start_character=characters_positions,
    end_line=lines_indices,
    end_character=characters_positions,
)
comment_filler_contents = _st.builds(
    FillerContent,
    _st.just(FillerKind.COMMENT_LINE),
    _st.from_regex(r'^//[^\n]*\n$', fullmatch=True),
) | _st.builds(
    FillerContent,
    _st.just(FillerKind.COMMENT_BLOCK),
    _st.from_regex(r'^/\*[^\*]*\*/$', fullmatch=True),
)
non_comment_filler_contents = _st.builds(
    FillerContent, _st.just(FillerKind.NEWLINE), _st.just('\n')
) | _st.builds(
    FillerContent,
    _st.just(FillerKind.WHITESPACE),
    _st.text(_st.characters(categories=['Zs']), min_size=1),
)
fillers_contents = non_comment_filler_contents | comment_filler_contents
non_comment_fillers = _st.builds(
    Filler, content=non_comment_filler_contents, position=substrings_positions
)
fillers = _st.builds(
    Filler, content=fillers_contents, position=substrings_positions
)


def _merge_consecutive_whitespaces_fillers(
    fillers: list[Filler],
) -> list[Filler]:
    return (
        [
            Filler(
                content=FillerContent(
                    FillerKind.WHITESPACE,
                    ''.join(filler.content.string for filler in fillers),
                ),
                position=SubstringPosition(
                    start_line=fillers[0].position.start_line,
                    start_character=fillers[0].position.start_character,
                    end_line=fillers[-1].position.end_line,
                    end_character=fillers[-1].position.end_character,
                ),
            )
        ]
        if len(fillers) > 1
        else fillers
    )


def _to_fillers_with_consecutive_whitespaces_merged(
    fillers: list[Filler],
) -> list[Filler]:
    return list(
        itertools.chain.from_iterable(
            (
                _merge_consecutive_whitespaces_fillers(list(group))
                if kind == FillerKind.WHITESPACE
                else list(group)
            )
            for kind, group in itertools.groupby(
                fillers, key=lambda filler: filler.content.kind
            )
        )
    )


def to_fillers_lists(
    elements: _st.SearchStrategy[Filler] = fillers,
    /,
    *,
    min_size: int = 0,
    max_size: int,
) -> _st.SearchStrategy[list[Filler]]:
    return _st.lists(elements, min_size=min_size, max_size=max_size).map(
        _to_fillers_with_consecutive_whitespaces_merged
    )


fillers_lists = to_fillers_lists(max_size=MAX_FILLERS_SIZE)
non_empty_fillers_lists = to_fillers_lists(
    min_size=1, max_size=MAX_FILLERS_SIZE
)
identifiers = _st.builds(
    Identifier,
    _st.from_regex(r'^[a-zA-Z_][a-zA-Z0-9_]*$', fullmatch=True),
    position=substrings_positions,
    fillers=fillers_lists,
)


def to_annotated_identifiers(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[AnnotatedIdentifier]:
    return _st.builds(
        AnnotatedIdentifier,
        identifiers,
        expressions,
        operator_position=substrings_positions,
        operator_fillers=fillers_lists,
    )


def to_assignments(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[Assignment]:
    return _st.builds(
        Assignment,
        expressions,
        expressions,
        operator_position=substrings_positions,
        operator_fillers=fillers_lists,
    )


@singledispatch
def _to_divisor(expression: Expression, _filler: Filler, /) -> Expression:
    raise TypeError(type(expression))


@_to_divisor.register(AnnotatedIdentifier)
def _(
    expression: AnnotatedIdentifier, filler: Filler, /
) -> AnnotatedIdentifier:
    identifier = _to_divisor(expression.identifier, filler)
    assert isinstance(identifier, Identifier), identifier
    return AnnotatedIdentifier(
        identifier,
        expression.annotation,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_divisor.register(Assignment)
def _(expression: Assignment, filler: Filler, /) -> Assignment:
    return Assignment(
        _to_divisor(expression.target, filler),
        expression.value,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_divisor.register(BinaryArithmeticOperation)
def _(
    expression: BinaryArithmeticOperation, filler: Filler, /
) -> BinaryArithmeticOperation:
    return BinaryArithmeticOperation(
        _to_divisor(expression.left, filler),
        expression.right,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_divisor.register(BinaryComparison)
def _(expression: BinaryComparison, filler: Filler, /) -> BinaryComparison:
    return BinaryComparison(
        _to_divisor(expression.left, filler),
        expression.right,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_divisor.register(Block)
def _(expression: Block, filler: Filler, /) -> Block:
    return Block(
        expression.statements,
        expression.expression,
        open_brace_position=expression.open_brace_position,
        close_brace_position=expression.close_brace_position,
        open_brace_fillers=_to_divisor_fillers(
            expression.open_brace_fillers, filler
        ),
        close_brace_fillers=expression.close_brace_fillers,
    )


@_to_divisor.register(Call)
def _(expression: Call, filler: Filler, /) -> Call:
    return Call(
        _to_divisor(expression.callable, filler),
        expression.arguments,
        open_parenthesis_position=expression.open_parenthesis_position,
        commas_positions=expression.commas_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        open_parenthesis_fillers=expression.open_parenthesis_fillers,
        commas_fillers=expression.commas_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
    )


@_to_divisor.register(Conditional)
def _(expression: Conditional, filler: Filler, /) -> Conditional:
    return Conditional(
        expression.antecedent,
        expression.consequent,
        expression.alternative,
        opener_position=expression.opener_position,
        alternative_opener_position=expression.alternative_opener_position,
        opener_fillers=_to_divisor_fillers(expression.opener_fillers, filler),
        alternative_opener_fillers=expression.alternative_opener_fillers,
    )


@_to_divisor.register(FunctionDefinition)
def _(expression: FunctionDefinition, filler: Filler, /) -> FunctionDefinition:
    return FunctionDefinition(
        expression.parameters,
        expression.return_type,
        expression.body,
        opener_position=expression.opener_position,
        open_parenthesis_position=expression.open_parenthesis_position,
        commas_positions=expression.commas_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        arrow_position=expression.arrow_position,
        opener_fillers=_to_divisor_fillers(expression.opener_fillers, filler),
        open_parenthesis_fillers=expression.open_parenthesis_fillers,
        commas_fillers=expression.commas_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
        arrow_fillers=expression.arrow_fillers,
    )


@_to_divisor.register(Grouping)
def _(expression: Grouping, filler: Filler, /) -> Grouping:
    return Grouping(
        expression.expression,
        open_parenthesis_position=expression.open_parenthesis_position,
        close_parenthesis_position=expression.close_parenthesis_position,
        open_parenthesis_fillers=_to_divisor_fillers(
            expression.open_parenthesis_fillers, filler
        ),
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
    )


@_to_divisor.register(Identifier)
def _(expression: Identifier, filler: Filler, /) -> Identifier:
    fillers = expression.fillers
    return Identifier(
        expression.string,
        position=expression.position,
        fillers=_to_divisor_fillers(fillers, filler),
    )


@_to_divisor.register(MemberAccess)
def _(expression: MemberAccess, filler: Filler, /) -> MemberAccess:
    return MemberAccess(
        _to_divisor(expression.object, filler),
        expression.member,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_divisor.register(NumericLiteral)
def _(expression: NumericLiteral, filler: Filler, /) -> NumericLiteral:
    return NumericLiteral(
        expression.value,
        expression.type_,
        position=expression.position,
        fillers=_to_divisor_fillers(expression.fillers, filler),
    )


@_to_divisor.register(Tuple)
def _(expression: Tuple, filler: Filler, /) -> Tuple:
    return Tuple(
        expression.elements,
        open_parenthesis_position=expression.open_parenthesis_position,
        commas_positions=expression.commas_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        open_parenthesis_fillers=_to_divisor_fillers(
            expression.open_parenthesis_fillers, filler
        ),
        commas_fillers=expression.commas_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
    )


@_to_divisor.register(UnaryArithmeticOperation)
def _(
    expression: UnaryArithmeticOperation, filler: Filler, /
) -> UnaryArithmeticOperation:
    return UnaryArithmeticOperation(
        expression.operand,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=_to_divisor_fillers(
            expression.operator_fillers, filler
        ),
    )


def _to_divisor_fillers(
    fillers: list[Filler], non_comment_filler: Filler
) -> list[Filler]:
    assert not _is_comment_filler(non_comment_filler), non_comment_filler
    return (
        [non_comment_filler] + fillers
        if fillers and _is_comment_filler(fillers[0])
        else fillers
    )


def _is_comment_filler(filler: Filler) -> bool:
    return filler.content.kind in (
        FillerKind.COMMENT_BLOCK,
        FillerKind.COMMENT_LINE,
    )


def to_binary_arithmetic_operations(
    operator: BinaryArithmeticOperator,
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[BinaryArithmeticOperation]:
    return _st.builds(
        BinaryArithmeticOperation,
        expressions,
        (
            _st.builds(_to_divisor, expressions, non_comment_fillers)
            if operator is BinaryDivisionOperator
            else expressions
        ),
        _st.just(operator),
        operator_position=substrings_positions,
        operator_fillers=fillers_lists,
    )


def to_binary_comparisons(
    operator: BinaryComparisonOperator,
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[BinaryComparison]:
    return _st.builds(
        BinaryComparison,
        expressions,
        expressions,
        _st.just(operator),
        operator_position=substrings_positions,
        operator_fillers=fillers_lists,
    )


def to_calls(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[Call]:
    return (
        _st.lists(expressions, max_size=MAX_EXPRESSIONS_SIZE)
        .flatmap(
            lambda arguments: _st.tuples(
                _st.just(arguments),
                _st.lists(
                    substrings_positions,
                    min_size=max(len(arguments) - 1, 0),
                    max_size=len(arguments),
                ),
            )
        )
        .flatmap(
            lambda arguments_with_commas_positions: _st.builds(
                Call,
                expressions,
                _st.just(arguments_with_commas_positions[0]),
                open_parenthesis_position=substrings_positions,
                commas_positions=_st.just(arguments_with_commas_positions[1]),
                close_parenthesis_position=substrings_positions,
                open_parenthesis_fillers=fillers_lists,
                commas_fillers=_st.lists(
                    fillers_lists,
                    min_size=len(arguments_with_commas_positions[1]),
                    max_size=len(arguments_with_commas_positions[1]),
                ),
                close_parenthesis_fillers=fillers_lists,
            )
        )
    )


def to_member_accesses(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[MemberAccess]:
    return _st.builds(
        MemberAccess,
        expressions,
        identifiers,
        operator_position=substrings_positions,
        operator_fillers=fillers_lists,
    )


def to_unary_arithmetic_operations(
    operator: UnaryArithmeticOperator,
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[UnaryArithmeticOperation]:
    return _st.builds(
        UnaryArithmeticOperation,
        expressions,
        _st.just(operator),
        operator_position=substrings_positions,
        operator_fillers=fillers_lists,
    )


def to_blocks(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[Block]:
    return _st.builds(
        Block,
        _st.lists(to_statements(expressions), max_size=MAX_STATEMENTS_SIZE),
        _st.none() | expressions,
        open_brace_position=substrings_positions,
        close_brace_position=substrings_positions,
        open_brace_fillers=fillers_lists,
        close_brace_fillers=fillers_lists,
    )


@singledispatch
def _to_antecedent(expression: Expression, _filler: Filler, /) -> Expression:
    raise TypeError(type(expression))


@_to_antecedent.register(AnnotatedIdentifier)
def _(
    expression: AnnotatedIdentifier, filler: Filler, /
) -> AnnotatedIdentifier:
    identifier = _to_antecedent(expression.identifier, filler)
    assert isinstance(identifier, Identifier), identifier
    return AnnotatedIdentifier(
        identifier,
        expression.annotation,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_antecedent.register(Assignment)
def _(expression: Assignment, filler: Filler, /) -> Assignment:
    return Assignment(
        _to_antecedent(expression.target, filler),
        expression.value,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_antecedent.register(Identifier)
def _(expression: Identifier, filler: Filler, /) -> Identifier:
    return Identifier(
        expression.string,
        position=expression.position,
        fillers=[filler] + expression.fillers,
    )


@_to_antecedent.register(NumericLiteral)
def _(expression: NumericLiteral, filler: Filler, /) -> NumericLiteral:
    return NumericLiteral(
        expression.value,
        expression.type_,
        position=expression.position,
        fillers=[filler] + expression.fillers,
    )


@_to_antecedent.register(UnaryArithmeticOperation)
def _(
    expression: UnaryArithmeticOperation, _filler: Filler, /
) -> UnaryArithmeticOperation:
    return expression


@_to_antecedent.register(BinaryArithmeticOperation)
def _(
    expression: BinaryArithmeticOperation, filler: Filler, /
) -> BinaryArithmeticOperation:
    return BinaryArithmeticOperation(
        _to_antecedent(expression.left, filler),
        expression.right,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_antecedent.register(BinaryComparison)
def _(expression: BinaryComparison, filler: Filler, /) -> BinaryComparison:
    return BinaryComparison(
        _to_antecedent(expression.left, filler),
        expression.right,
        expression.operator,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


@_to_antecedent.register(Block)
def _(expression: Block, _filler: Filler, /) -> Block:
    return expression


@_to_antecedent.register(Grouping)
def _(expression: Grouping, _filler: Filler, /) -> Grouping:
    return expression


@_to_antecedent.register(Tuple)
def _(expression: Tuple, _filler: Filler, /) -> Tuple:
    return expression


@_to_antecedent.register(Call)
def _(expression: Call, filler: Filler, /) -> Call:
    return Call(
        _to_antecedent(expression.callable, filler),
        expression.arguments,
        open_parenthesis_position=expression.open_parenthesis_position,
        commas_positions=expression.commas_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        open_parenthesis_fillers=expression.open_parenthesis_fillers,
        commas_fillers=expression.commas_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
    )


@_to_antecedent.register(Conditional)
def _(expression: Conditional, filler: Filler, /) -> Conditional:
    return Conditional(
        expression.antecedent,
        expression.consequent,
        expression.alternative,
        opener_position=expression.opener_position,
        alternative_opener_position=expression.alternative_opener_position,
        opener_fillers=[filler] + expression.opener_fillers,
        alternative_opener_fillers=expression.alternative_opener_fillers,
    )


@_to_antecedent.register(FunctionDefinition)
def _(expression: FunctionDefinition, filler: Filler, /) -> FunctionDefinition:
    return FunctionDefinition(
        expression.parameters,
        expression.return_type,
        expression.body,
        opener_position=expression.opener_position,
        open_parenthesis_position=expression.open_parenthesis_position,
        commas_positions=expression.commas_positions,
        close_parenthesis_position=expression.close_parenthesis_position,
        arrow_position=expression.arrow_position,
        opener_fillers=[filler] + expression.opener_fillers,
        open_parenthesis_fillers=expression.open_parenthesis_fillers,
        commas_fillers=expression.commas_fillers,
        close_parenthesis_fillers=expression.close_parenthesis_fillers,
        arrow_fillers=expression.arrow_fillers,
    )


@_to_antecedent.register(MemberAccess)
def _(expression: MemberAccess, filler: Filler, /) -> MemberAccess:
    return MemberAccess(
        _to_antecedent(expression.object, filler),
        expression.member,
        operator_position=expression.operator_position,
        operator_fillers=expression.operator_fillers,
    )


def to_antecedents(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[Expression]:
    return _st.builds(_to_antecedent, expressions, fillers)


def to_conditionals(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[Conditional]:
    antecedents = to_antecedents(expressions)
    blocks = to_blocks(expressions)
    return _st.builds(
        Conditional,
        antecedents,
        blocks,
        _st.none(),
        opener_position=substrings_positions,
        alternative_opener_position=_st.none(),
        opener_fillers=fillers_lists,
        alternative_opener_fillers=_st.builds(list),
    ) | _st.builds(
        Conditional,
        expressions,
        blocks,
        blocks
        | _st.recursive(
            _st.builds(
                Conditional,
                antecedents,
                blocks,
                _st.none(),
                opener_position=substrings_positions,
                alternative_opener_position=_st.none(),
                opener_fillers=non_empty_fillers_lists,
                alternative_opener_fillers=_st.builds(list),
            ),
            lambda step: _st.builds(
                Conditional,
                antecedents,
                blocks,
                blocks | step,
                opener_position=substrings_positions,
                alternative_opener_position=substrings_positions,
                opener_fillers=non_empty_fillers_lists,
                alternative_opener_fillers=fillers_lists,
            ),
            max_leaves=4,
        ),
        opener_position=substrings_positions,
        alternative_opener_position=substrings_positions,
        opener_fillers=fillers_lists,
        alternative_opener_fillers=fillers_lists,
    )


@_st.composite
def to_function_definitions(
    draw: _st.DrawFn, expressions: _st.SearchStrategy[Expression]
) -> FunctionDefinition:
    parameters = draw(
        _st.lists(
            to_annotated_identifiers(expressions),
            max_size=MAX_EXPRESSIONS_SIZE,
        )
    )
    commas_positions = draw(
        _st.lists(
            substrings_positions,
            min_size=max(len(parameters) - 1, 0),
            max_size=len(parameters),
        )
    )
    return FunctionDefinition(
        parameters,
        draw(expressions),
        draw(to_blocks(expressions)),
        opener_position=draw(substrings_positions),
        open_parenthesis_position=draw(substrings_positions),
        commas_positions=commas_positions,
        close_parenthesis_position=draw(substrings_positions),
        arrow_position=draw(substrings_positions),
        opener_fillers=draw(fillers_lists),
        open_parenthesis_fillers=draw(fillers_lists),
        commas_fillers=draw(
            _st.lists(
                fillers_lists,
                min_size=len(commas_positions),
                max_size=len(commas_positions),
            )
        ),
        close_parenthesis_fillers=draw(fillers_lists),
        arrow_fillers=draw(fillers_lists),
    )


def to_groupings(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[Grouping]:
    return _st.builds(
        Grouping,
        expressions,
        open_parenthesis_position=substrings_positions,
        close_parenthesis_position=substrings_positions,
        open_parenthesis_fillers=fillers_lists,
        close_parenthesis_fillers=fillers_lists,
    )


@_st.composite
def to_tuples(
    draw: _st.DrawFn, expressions: _st.SearchStrategy[Expression]
) -> Tuple:
    elements = draw(_st.lists(expressions))
    commas_positions = draw(
        _st.lists(
            substrings_positions,
            min_size=(1 if len(elements) == 1 else max(len(elements) - 1, 0)),
            max_size=len(elements),
        )
    )
    return Tuple(
        elements,
        open_parenthesis_position=draw(substrings_positions),
        commas_positions=commas_positions,
        close_parenthesis_position=draw(substrings_positions),
        open_parenthesis_fillers=draw(fillers_lists),
        commas_fillers=draw(
            _st.lists(
                fillers_lists,
                min_size=len(commas_positions),
                max_size=len(commas_positions),
            )
        ),
        close_parenthesis_fillers=draw(fillers_lists),
    )


def to_statements(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[Statement]:
    return to_expression_statements(expressions)


def to_expression_statements(
    expressions: _st.SearchStrategy[Expression],
) -> _st.SearchStrategy[ExpressionStatement]:
    return _st.builds(
        ExpressionStatement,
        expressions,
        semicolon_position=substrings_positions,
        semicolon_fillers=fillers_lists,
    )
