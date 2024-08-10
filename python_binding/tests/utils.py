import builtins
import inspect
import pickle
from collections.abc import Callable
from itertools import zip_longest
from typing import (
    Annotated,
    Any,
    Final,
    Protocol,
    Self,
    TypeVar,
    get_args,
    get_origin,
)


def equivalence(left: bool, right: bool, /) -> bool:
    return left is right


def implication(antecedent: bool, consequent: bool, /) -> bool:
    return not antecedent or consequent


def to_class_signature(cls: type[Any], /) -> inspect.Signature | None:
    try:
        return inspect.signature(cls)
    except (TypeError, ValueError):
        return None


_T = TypeVar('_T')


def to_self_referential_class_fields(cls: type[_T], /) -> dict[str, _T]:
    return {
        name: value
        for name, value in vars(cls).items()
        if isinstance(value, cls)
    }


def to_typeless_signature(
    signature: inspect.Signature, /
) -> inspect.Signature | None:
    try:
        return signature.replace(
            parameters=[
                parameter.replace(annotation=inspect.Parameter.empty)
                for parameter in signature.parameters.values()
            ],
            return_annotation=inspect.Parameter.empty,
        )
    except ValueError:
        return None


# See Include/object.h
TPFLAGS_BASETYPE: Final[int] = 1 << 10


def is_class_final(cls: type[Any], /) -> bool:
    return not (cls.__flags__ & TPFLAGS_BASETYPE)


def is_private_object_name(value: str, /) -> bool:
    return value.startswith('_')


def pack(callable_: Callable[..., _T]) -> Callable[[tuple[Any, ...]], _T]:
    def wrapped(args: tuple[Any, ...]) -> _T:
        return callable_(*args)

    return wrapped


def to_base_annotation(annotation: Any, /) -> Any | None:
    result = get_origin(annotation)
    if result is Annotated:
        result = get_origin(get_args(annotation)[0])
    return result


class _Sortable(Protocol):
    def __lt__(self, other: Self, /) -> bool: ...


_SortableT = TypeVar('_SortableT', bound=_Sortable)


def escape_if_built_in_name(name: str, /) -> str:
    return f'{name}_' if name in vars(builtins) else name


def to_sorted_lists_diff(
    left: list[_SortableT], right: list[_SortableT], /
) -> list[tuple[_SortableT | None, _SortableT | None]]:
    result: list[tuple[_SortableT | None, _SortableT | None]] = []
    left_elements, right_elements = iter(left), iter(right)
    try:
        left_element = next(left_elements)
    except StopIteration:
        result.extend(
            (None, right_element) for right_element in right_elements
        )
    else:
        while True:
            try:
                while (right_element := next(right_elements)) < left_element:
                    result.append((None, right_element))
            except StopIteration:
                result.append((left_element, None))
                result.extend(
                    (left_element, None) for left_element in left_elements
                )
                break
            else:
                if right_element == left_element:
                    try:
                        left_element = next(left_elements)
                    except StopIteration:
                        result.extend(
                            (None, right_element)
                            for right_element in right_elements
                        )
                        break
                    else:
                        continue
                result.append((left_element, None))
                assert left_element < right_element, (
                    left_element,
                    right_element,
                )
                try:
                    while (
                        left_element := next(left_elements)
                    ) < right_element:
                        result.append((left_element, None))
                except StopIteration:
                    result.append((None, right_element))
                    result.extend(
                        (None, right_element)
                        for right_element in right_elements
                    )
                    break
                else:
                    if left_element == right_element:
                        try:
                            left_element = next(left_elements)
                        except StopIteration:
                            result.extend(
                                (None, right_element)
                                for right_element in right_elements
                            )
                            break
                        else:
                            continue
                    result.append((None, right_element))
                    assert right_element < left_element, (
                        left_element,
                        right_element,
                    )
    return result


def natural_sorting_key(value: str, /) -> list[int | str]:
    result: list[int | str] = []
    part_index = 0
    while part_index < len(value):
        part_start_index = part_index
        is_integer_part = value[part_index].isdigit()
        while (part_index := part_index + 1) < len(value) and value[
            part_index
        ].isdigit() is is_integer_part:
            pass
        part = value[part_start_index:part_index]
        result.append(int(part) if is_integer_part else part)
    return result


def pickling_round_trip(value: Any, /) -> Any:
    return pickle.loads(pickle.dumps(value))


def to_unsorted_list_elements(
    value: list[_T], /, *, key: Callable[[_T], _SortableT]
) -> list[tuple[_T, _T]]:
    return [
        (element, sorted_element)
        for element, sorted_element in zip(
            value, sorted(value, key=key), strict=False
        )
        if element != sorted_element
    ]


def to_lists_different_elements(
    left: list[_T], right: list[_T], /
) -> list[tuple[_T | None, _T | None]]:
    return [
        (left_element, right_element)
        for left_element, right_element in zip_longest(left, right)
        if left_element != right_element
    ]
