from enum import Enum
from io import StringIO
import itertools
from typing import Generator

from . import year2024

day = year2024.day(2)


@day.generator
def generator(input: str) -> Generator[list[int], None, None]:
    for row in StringIO(input).readlines():
        parsed = [int(x) for x in row.strip().split()]
        yield parsed


class PairDisposition(Enum):
    DESCENDING = -1
    ASCENDING = 1
    INVALID = 0

    @property
    def is_valid(self):
        return self != PairDisposition.INVALID


def test_pair(x: int, y: int) -> PairDisposition:
    delta = abs(x - y)
    if delta < 1 or delta > 3:
        return PairDisposition.INVALID
    if x > y:
        return PairDisposition.DESCENDING
    return PairDisposition.ASCENDING


def valid_part1_report(row: list[int]) -> bool:
    dis = [test_pair(x, y) for x, y in itertools.pairwise(row)]
    return all(x.is_valid for x in dis) and all(x == dis[0] for x in dis)


@day.part1
def part1(input: list[list[int]]) -> int:
    return sum(1 if valid_part1_report(row) else 0 for row in input)


def valid_part2_report(row: list[int]) -> bool:
    # # Figure out if we're ascending or descending
    # # Remember the case where every entry is invalid
    # tests = (test_pair(x, y) for x, y in itertools.pairwise(row))
    # direction = next((i for i in tests if i.is_valid), PairDisposition.INVALID)

    # # If nothing is valid, just bail.  There's no saving this.
    # if not direction.is_valid:
    #     return False

    # for idx, (x, y) in enumerate(itertools.pairwise(row)):
    #     dis = test_pair(x, y)
    #     if dis != direction:
    #         # Try once again without that element

    if valid_part1_report(row):
        return True
    for i in range(len(row)):
        adjusted_row = row[:i] + row[i + 1 :]
        if valid_part1_report(adjusted_row):
            return True
    return False


@day.part2
def part2(input: list[list[int]]) -> int:
    return sum(1 if valid_part2_report(x) else 0 for x in input)


@day.test(part1=2, part2=4)
def test():
    return """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""
