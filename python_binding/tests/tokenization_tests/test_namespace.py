import importlib
import inspect
import sys
import typing as t
from types import ModuleType

import pytest
from _pytest.monkeypatch import MonkeyPatch

from tests.utils import (
    is_class_final,
    is_private_object_name,
    to_base_annotation,
    to_class_signature,
    to_typeless_signature,
)


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
    module_namespace: dict[str, t.Any],
    type_annotated_module_namespace: dict[str, t.Any],
) -> None:
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
        name: value
        for name, value in module_namespace.items()
        if (
            not is_private_object_name(name)
            and (
                inspect.isfunction(value)
                or inspect.isbuiltin(value)
                or inspect.isclass(value)
            )
            and getattr(value, '__module__', None) != module.__name__
        )
    } == {}


def test_constructible_classes(
    module_namespace: dict[str, t.Any],
    type_annotated_module_namespace: dict[str, t.Any],
) -> None:
    classes = {
        name: value
        for name, value in module_namespace.items()
        if (
            inspect.isclass(value)
            and value.__module__ == module_namespace['__name__']
        )
    }
    type_annotated_classes = {
        name: value
        for name, value in type_annotated_module_namespace.items()
        if (
            inspect.isclass(value)
            and value.__module__ == type_annotated_module_namespace['__name__']
        )
    }
    constructible_classes_signatures = {
        value: signature
        for value in classes.values()
        if (signature := to_class_signature(value)) is not None
    }
    type_annotated_classes_signatures = {
        value: signature
        for value in type_annotated_classes.values()
        if (signature := to_class_signature(value)) is not None
    }

    assert constructible_classes_signatures
    assert classes.keys() == type_annotated_classes.keys()
    assert {cls.__qualname__ for cls in constructible_classes_signatures} - {
        cls.__qualname__ for cls in type_annotated_classes_signatures
    } == set()
    assert [
        class_name
        for class_name in (
            {cls.__qualname__ for cls in type_annotated_classes_signatures}
            - {cls.__qualname__ for cls in constructible_classes_signatures}
        )
        if (
            not is_class_final(module_namespace[class_name])
            and not inspect.isabstract(
                type_annotated_module_namespace[class_name]
            )
        )
    ] == []
    assert [
        cls
        for cls in constructible_classes_signatures
        if (
            is_class_final(cls)
            is not getattr(
                type_annotated_module_namespace[cls.__qualname__],
                '__final__',
                False,
            )
        )
    ] == []
    assert [
        cls
        for cls, signature in constructible_classes_signatures.items()
        if (
            sorted(
                parameter_name
                for parameter_name in signature.parameters
                if not is_private_object_name(parameter_name)
            )
            != sorted(
                name
                for name, field in vars(cls).items()
                if inspect.isgetsetdescriptor(field)
            )
        )
    ] == []
    assert [
        cls
        for cls, signature in constructible_classes_signatures.items()
        if (
            (
                (
                    type_annotated_cls := type_annotated_module_namespace.get(
                        cls.__qualname__
                    )
                )
                is None
            )
            or (
                [
                    parameter_name
                    for parameter_name in signature.parameters
                    if not is_private_object_name(parameter_name)
                ]
                != [
                    field_name
                    for field_name, field_annotation in t.get_type_hints(
                        type_annotated_cls, type_annotated_module_namespace
                    ).items()
                    if to_base_annotation(field_annotation) is not t.ClassVar
                ]
            )
        )
    ] == []
    assert [
        cls
        for cls, signature in constructible_classes_signatures.items()
        if (
            (
                type_annotated_cls := type_annotated_module_namespace.get(
                    cls.__qualname__
                )
            )
            is None
            or (
                (
                    type_annotated_signature := to_class_signature(
                        type_annotated_cls
                    )
                )
                is None
            )
            or signature != to_typeless_signature(type_annotated_signature)
        )
    ] == []


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
