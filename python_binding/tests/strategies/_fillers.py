from itertools import chain as _chain, groupby as _groupby

from az.parsing import (
    Filler as _Filler,
    FillerContent as _FillerContent,
    FillerKind as _FillerKind,
)
from az.tokenization import SubstringPosition as _SubstringPosition
from hypothesis import strategies as _st

from . import _filler_contents as _filler_contents
from ._positions import (
    substring_position_strategy as _substring_position_strategy,
)

_MAX_FILLERS_SIZE = 4
non_comment_filler_strategy = _st.builds(
    _Filler,
    _filler_contents.non_comment_filler_content_strategy,
    position=_substring_position_strategy,
)
filler_strategy = _st.builds(
    _Filler,
    _filler_contents.filler_content_strategy,
    position=_substring_position_strategy,
)


def _merge_consecutive_whitespace_fillers(
    fillers: list[_Filler], /
) -> _Filler:
    return (
        _Filler(
            _FillerContent(
                _FillerKind.WHITESPACE,
                ''.join(str(filler.content) for filler in fillers),
            ),
            position=_SubstringPosition(
                start=fillers[0].position.start, end=fillers[-1].position.end
            ),
        )
        if len(fillers) > 1
        else fillers[0]
    )


def _to_filler_list_strategy(
    elements: _st.SearchStrategy[_Filler] = filler_strategy,
    /,
    *,
    min_size: int = 0,
    max_size: int,
) -> _st.SearchStrategy[list[_Filler]]:
    return (
        _st.lists(elements, min_size=min_size, max_size=max_size)
        .map(_to_filler_list_with_consecutive_whitespaces_merged)
        .map(_to_filler_list_with_comment_lines_followed_by_newlines)
    )


def _to_filler_list_with_comment_lines_followed_by_newlines(
    fillers: list[_Filler], /
) -> list[_Filler]:
    result = []
    for filler in fillers:
        result.append(filler)
        if filler.content.kind is _FillerKind.COMMENT_LINE:
            result.append(
                _Filler(
                    _FillerContent(_FillerKind.NEWLINE),
                    position=_SubstringPosition(
                        start=filler.position.end, end=filler.position.end
                    ),
                )
            )
    return result


def _to_filler_list_with_consecutive_whitespaces_merged(
    fillers: list[_Filler], /
) -> list[_Filler]:
    return list(
        _chain.from_iterable(
            (
                [_merge_consecutive_whitespace_fillers(list(group))]
                if kind is _FillerKind.WHITESPACE
                else list(group)
            )
            for kind, group in _groupby(
                fillers, key=lambda filler: filler.content.kind
            )
        )
    )


filler_kind_strategy = _st.sampled_from(
    [
        _FillerKind.COMMENT_BLOCK,
        _FillerKind.COMMENT_LINE,
        _FillerKind.NEWLINE,
        _FillerKind.WHITESPACE,
    ]
)
filler_list_strategy = _to_filler_list_strategy(max_size=_MAX_FILLERS_SIZE)
non_empty_filler_list_strategy = _to_filler_list_strategy(
    min_size=1, max_size=_MAX_FILLERS_SIZE
)
