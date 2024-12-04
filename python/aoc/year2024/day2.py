from collections.abc import Callable
from io import StringIO
import itertools
import math
import operator
from typing import Generator
from . import year2024

day = year2024.day(2)


@day.generator
def generator(input: str) -> Generator[list[int], None, None]:
    for row in StringIO(input).readlines():
        parsed = [int(x) for x in row.strip().split()]
        yield parsed


def is_increasing(row: list[int]) -> bool:
    return all(x < y for x, y in itertools.pairwise(row))


def is_decreasing(row: list[int]) -> bool:
    return all(x > y for x, y in itertools.pairwise(row))


def is_safe_single_step(x: int, y: int) -> bool:
    delta = abs(x - y)
    return delta >= 1 and delta <= 3


def is_safe_step(row: list[int]) -> bool:
    return all(is_safe_single_step(x, y) for x, y in itertools.pairwise(row))


def is_safe_report(row: list[int]) -> bool:
    if not is_increasing(row) and not is_decreasing(row):
        return False
    return is_safe_step(row)


@day.part1
def part1(input: list[list[int]]) -> int:
    return sum(1 if is_safe_report(x) else 0 for x in input)


def is_faulted_template(
    compare: Callable[[int, int], bool], assertion: Callable[[list[int]], bool]
) -> Callable[[list[int]], bool]:
    def inner(row: list[int]) -> bool:
        for idx, (x, y) in enumerate(itertools.pairwise(row)):
            if compare(x, y):
                return assertion(row[:idx] + row[idx + 1 :])
        return True

    return inner


is_faulted_increasing = is_faulted_template(operator.ge, is_increasing)
is_faulted_decreasing = is_faulted_template(operator.le, is_decreasing)
is_faulted_safe_step = is_faulted_template(
    lambda x, y: not is_safe_single_step(x, y), is_safe_step
)


def is_faulted_safe_report(row: list[int]) -> bool:
    if not is_faulted_increasing(row) and not is_faulted_decreasing(row):
        return False
    return is_faulted_safe_step(row)


@day.part2
def part2(input: list[list[int]]) -> int:
    return sum(1 if is_faulted_safe_report(x) else 0 for x in input)


@day.test(part1=2, part2=4)
def test():
    return """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""
