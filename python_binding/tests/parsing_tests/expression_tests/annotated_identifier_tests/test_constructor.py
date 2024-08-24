from az.parsing import AnnotatedIdentifier, Expression, Filler, Identifier
from az.tokenization import SubstringPosition
from hypothesis import given

from tests.strategies import (
    annotated_identifier_strategy,
    annotation_strategy,
    filler_list_strategy,
    identifier_strategy,
    substring_position_strategy,
)


@given(
    identifier_strategy,
    annotation_strategy,
    substring_position_strategy,
    filler_list_strategy,
)
def test_basic(
    identifier: Identifier,
    annotation: Expression,
    operator_position: SubstringPosition,
    operator_fillers: list[Filler],
) -> None:
    result = AnnotatedIdentifier(
        identifier,
        annotation,
        operator_position=operator_position,
        operator_fillers=operator_fillers,
    )

    assert isinstance(result, AnnotatedIdentifier)
    assert result.identifier == identifier
    assert result.annotation == annotation
    assert result.operator_position == operator_position
    assert result.operator_fillers == operator_fillers


@given(annotated_identifier_strategy)
def test_round_trip(annotated_identifier: AnnotatedIdentifier) -> None:
    result = AnnotatedIdentifier(
        annotated_identifier.identifier,
        annotated_identifier.annotation,
        operator_position=annotated_identifier.operator_position,
        operator_fillers=annotated_identifier.operator_fillers,
    )

    assert result == annotated_identifier
