import os
import time
from collections.abc import Callable, Generator, Iterator
from datetime import timedelta
from typing import Any, ParamSpec, TypeVar, cast
from unittest.mock import patch

import pytest
from hypothesis import HealthCheck, settings


@pytest.fixture(scope='session', autouse=True)
def patch_hypothesis() -> Generator[None, None, None]:
    from functools import wraps

    from hypothesis import strategies as st

    _T = TypeVar('_T')

    class _StrategyWrapper(st.SearchStrategy[_T]):
        """
        Sole purpose of this wrapper is to avoid memory errors
        by providing simple `__repr__`.
        """

        def __init__(self, wrapped: st.SearchStrategy[_T]) -> None:
            super().__init__()  # type: ignore
            self.__wrapped = wrapped

        @property
        def branches(self) -> list[st.SearchStrategy[_T]]:
            return self.__wrapped.branches

        def do_draw(self, data: Any) -> _T:
            return self.__wrapped.do_draw(data)

        def do_validate(self) -> None:
            self.__wrapped.do_validate()  # type: ignore

        def validate(self) -> None:
            self.__wrapped.validate()

        def __repr__(self) -> str:
            return object.__repr__(self)

    _Params = ParamSpec('_Params')

    def _wrap(
        function: Callable[_Params, st.SearchStrategy[_T]],
    ) -> Callable[_Params, _StrategyWrapper[_T]]:
        @wraps(function)
        def wrapped(*args: Any, **kwargs: Any) -> _StrategyWrapper[_T]:
            return _StrategyWrapper(function(*args, **kwargs))

        return wrapped

    with patch.multiple(st, builds=_wrap(st.builds), one_of=_wrap(st.one_of)):
        yield


on_ci = bool(os.getenv('CI', False))
max_examples = (
    settings.default.max_examples // 5
    if on_ci
    else settings.default.max_examples
)
settings.register_profile(
    'default',
    deadline=None,
    max_examples=max_examples,
    suppress_health_check=[HealthCheck.filter_too_much, HealthCheck.too_slow],
)

# FIXME:
#  workaround until https://github.com/pytest-dev/pluggy/issues/191 is fixed
hookimpl = cast(Callable[..., Callable[..., None]], pytest.hookimpl)

if on_ci:
    time_left = timedelta(hours=1)

    @hookimpl(tryfirst=True)
    def pytest_runtest_call(item: pytest.Function) -> None:
        set_deadline = settings(deadline=time_left / max_examples)
        item.obj = set_deadline(item.obj)

    @pytest.fixture(scope='function', autouse=True)
    def time_function_call() -> Iterator[None]:
        start = time.monotonic()
        try:
            yield
        finally:
            duration = timedelta(seconds=time.monotonic() - start)
            global time_left
            time_left = max(duration, time_left) - duration


@hookimpl(trylast=True)
def pytest_sessionfinish(
    session: pytest.Session, exitstatus: pytest.ExitCode
) -> None:
    if exitstatus == pytest.ExitCode.NO_TESTS_COLLECTED:
        session.exitstatus = pytest.ExitCode.OK
