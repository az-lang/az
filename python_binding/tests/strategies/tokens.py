from hypothesis import strategies as _st

from az.parsing import Script
from az.tokenization import tokenize_string

from .scripts import non_empty_scripts, scripts, scripts_strings

tokens = non_empty_scripts.map(Script.tokenize).flatmap(_st.sampled_from)
tokens_contents = tokens.map(lambda token: token.content)
parseable_tokens_lists = scripts.map(Script.tokenize) | scripts_strings.map(
    tokenize_string
)
