from az.parsing import FillerKind
from hypothesis import given

from tests.strategies import filler_kind_strategy
from tests.utils import pickling_round_trip


@given(filler_kind_strategy)
def test_round_trip(filler_kind: FillerKind) -> None:
    assert pickling_round_trip(filler_kind) is filler_kind
