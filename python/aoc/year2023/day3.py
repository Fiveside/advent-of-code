from . import year2023
from io import StringIO
import re
from functools import reduce
import operator
import typing

day = year2023.day(3)


@day.generator
def generator(input: str) -> list[str]:
    return [x.strip() for x in StringIO(input).readlines()]


def is_symbol(c: str) -> bool:
    # Its a symbol if its not alphabetical, numeric, or a period.
    # I realize this doesn't cover non-printables, but those aren't
    # part of the input?
    return not c.isalnum() and c != "."


class Point(typing.NamedTuple):
    line: int
    col: int


class SurrogateKey(Point):
    pass


class PartNumber(typing.NamedTuple):
    sk: SurrogateKey
    value: int


def tokenize(
    input: list[str],
) -> tuple[dict[Point, PartNumber], set[Point]]:
    numbers = dict()
    symbols = set()
    for line_num, line in enumerate(input):
        for match in re.finditer(r"\d+", line):
            num = int(match.group())
            sk = SurrogateKey(line_num, match.start())
            for col_num in range(*match.span()):
                numbers[Point(line_num, col_num)] = PartNumber(sk, num)
        for col_num, item in enumerate(line):
            if is_symbol(item):
                symbols.add(Point(line_num, col_num))
    return numbers, symbols


DELTAS = (
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    # Free space!
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
)


def adjacent_number_entries(
    x: int, y: int, db: dict[Point, PartNumber]
) -> list[PartNumber]:
    for dx, dy in DELTAS:
        key = (x + dx, y + dy)
        if key in db:
            yield db[key]


@day.part1
def part1(input: list[str]) -> int:
    # First, tokenize symbols and numbers and add them to a lookup dictionary.
    # Then, reconcile
    numbers, symbols = tokenize(input)

    # Now take each symbol, and collect adjacent numbers
    part_numbers = dict()
    for line, col in symbols:
        for sk, num in adjacent_number_entries(line, col, numbers):
            part_numbers[sk] = num

    # Now add up all the part numbers
    return sum(part_numbers.values())


@day.part2
def part2(input: list[str]) -> int:
    numbers, symbols = tokenize(input)
    gear_ratios = []
    for line, col in symbols:
        adjacents = dict(adjacent_number_entries(line, col, numbers))
        if len(adjacents) == 2:
            gear_ratios.append(reduce(operator.mul, adjacents.values()))
    return sum(gear_ratios)


@day.test(4361, 467835)
def test():
    return """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""
