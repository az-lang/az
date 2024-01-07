import importlib
import inspect
import sys
import typing as t
from types import ModuleType

import pytest
from _pytest.monkeypatch import MonkeyPatch


@pytest.fixture(scope='module')
def module() -> ModuleType:
    from az import tokenization

    return tokenization


@pytest.fixture(scope='module')
def module_namespace(module: ModuleType) -> dict[str, t.Any]:
    return vars(module)


@pytest.fixture(scope='function')
def type_annotated_module(
    module: ModuleType, monkeypatch: MonkeyPatch
) -> ModuleType:
    with monkeypatch.context() as patch:
        patch.setattr(sys, 'modules', {})
        patch.setattr(t, 'TYPE_CHECKING', True)
        return importlib.import_module(module.__name__)


@pytest.fixture(scope='function')
def type_annotated_module_namespace(
    type_annotated_module: ModuleType,
) -> dict[str, t.Any]:
    return vars(type_annotated_module)


def test_type_annotations(
    module: ModuleType,
    module_namespace: dict[str, t.Any],
    type_annotated_module: ModuleType,
    type_annotated_module_namespace: dict[str, t.Any],
) -> None:
    type_annotated_module_namespace = vars(type_annotated_module)

    assert type_annotated_module_namespace.keys() == module_namespace.keys()
    assert [
        name
        for name in (
            type_annotated_module_namespace.keys() & module_namespace.keys()
        )
        if (
            type(type_annotated_module_namespace[name])
            is not type(module_namespace[name])
        )
        and (
            inspect.isfunction(type_annotated_module_namespace[name])
            is not inspect.isbuiltin(module_namespace[name])
        )
    ] == []


def test_members_modules_names(
    module: ModuleType, module_namespace: dict[str, t.Any]
) -> None:
    assert {
        key: value
        for key, value in module_namespace.items()
        if (
            (
                inspect.isfunction(value)
                or inspect.isbuiltin(value)
                or inspect.isclass(value)
            )
            and getattr(value, '__module__', None) != module.__name__
        )
    } == {}


def test_lexical_error_subclasses(
    module: ModuleType, module_namespace: dict[str, t.Any]
) -> None:
    subclasses = module.LexicalError.__subclasses__()

    assert subclasses
    assert [
        cls
        for cls in subclasses
        if module_namespace.get(cls.__qualname__) is not cls
    ] == []
