from az.parsing import FillerKind
from hypothesis import given

from tests.strategies import filler_kind_strategy


@given(filler_kind_strategy)
def test_round_trip(filler_kind: FillerKind) -> None:
    result = repr(filler_kind)

    assert eval(result) is filler_kind
