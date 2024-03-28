from hypothesis import strategies as _st
from hypothesis.extra.lark import from_lark
from lark import Lark

from az.parsing import Script

from tests.utils import tokens_to_string
from .factories import fillers_lists
from .grammar import common_grammar
from .statements import statements

scripts = _st.builds(Script, _st.lists(statements), fillers=fillers_lists)
non_empty_scripts = _st.builds(
    Script, _st.lists(statements, min_size=1), fillers=fillers_lists
)
scripts_strings = scripts.map(Script.tokenize).map(
    tokens_to_string
) | from_lark(Lark('start: statement*\n' f'{common_grammar}'))
