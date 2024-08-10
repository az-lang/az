from az.parsing import Associativity
from hypothesis import given

from tests.strategies import associativity_strategy
from tests.utils import pickling_round_trip


@given(associativity_strategy)
def test_round_trip(associativity: Associativity) -> None:
    assert pickling_round_trip(associativity) is associativity
