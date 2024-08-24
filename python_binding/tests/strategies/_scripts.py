from az.parsing import Filler, Script, Statement
from hypothesis import strategies as _st

from tests.tokenization_tests.utils import tokens_to_string

from ._fillers import filler_list_strategy
from ._statements import statement_strategy


def to_validated_script(
    statements: list[Statement], /, *, fillers: list[Filler]
) -> Script:
    result = Script(statements, fillers=fillers)
    result.validate_contents()
    return result


script_strategy = _st.builds(
    to_validated_script,
    _st.lists(statement_strategy),
    fillers=filler_list_strategy,
)
script_string_strategy = script_strategy.map(Script.tokenize).map(
    tokens_to_string
)
