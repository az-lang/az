import os
import time
import typing as t
from datetime import timedelta

import pytest
from hypothesis import HealthCheck, settings


@pytest.fixture(scope='session', autouse=True)
def patch_hypothesis() -> t.Generator[None, None, None]:
    from functools import wraps

    from hypothesis import strategies as st

    _T = t.TypeVar('_T')

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

        def do_draw(self, data: t.Any) -> _T:
            return self.__wrapped.do_draw(data)

        def do_validate(self) -> None:
            self.__wrapped.do_validate()  # type: ignore

        def validate(self) -> None:
            self.__wrapped.validate()

        def __repr__(self) -> str:
            return object.__repr__(self)

    _Params = t.ParamSpec('_Params')

    def _wrap(
        function: t.Callable[_Params, st.SearchStrategy[_T]],
    ) -> t.Callable[_Params, _StrategyWrapper[_T]]:
        @wraps(function)
        def wrapped(*args: t.Any, **kwargs: t.Any) -> _StrategyWrapper[_T]:
            return _StrategyWrapper(function(*args, **kwargs))

        return wrapped

    old = st.builds
    st.builds = _wrap(old)
    try:
        yield
    finally:
        st.builds = old


on_ci = bool(os.getenv('CI', False))
max_examples = (
    settings.default.max_examples // 10
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
hookimpl = t.cast(t.Callable[..., t.Callable[..., None]], pytest.hookimpl)

if on_ci:
    time_left = timedelta(hours=1)

    @hookimpl(tryfirst=True)
    def pytest_runtest_call(item: pytest.Function) -> None:
        set_deadline = settings(deadline=time_left / max_examples)
        item.obj = set_deadline(item.obj)

    @pytest.fixture(scope='function', autouse=True)
    def time_function_call() -> t.Iterator[None]:
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
