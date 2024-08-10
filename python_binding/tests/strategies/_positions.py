import sys

from az.tokenization import (
    ByteCount,
    CharacterPosition,
    SubstringPosition,
    Utf8Count,
)
from hypothesis import strategies as _st

byte_count_strategy = _st.builds(
    ByteCount, _st.integers(0, int(ByteCount.MAX))
)
utf_8_count_strategy = _st.builds(
    Utf8Count, _st.integers(0, int(Utf8Count.MAX))
)
character_position_strategy = _st.builds(
    CharacterPosition, byte=byte_count_strategy, utf_8=utf_8_count_strategy
)
line_index_strategy = _st.integers(0, sys.maxsize)
substring_position_strategy = _st.builds(
    SubstringPosition,
    start=character_position_strategy,
    end=character_position_strategy,
)
