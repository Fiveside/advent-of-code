from . import year2023
from dataclasses import dataclass
from io import StringIO
from typing import Generator
from .utils import sliding_window
from functools import cached_property

day = year2023.day(13)


@dataclass(frozen=True)
class Grid:
    rows: list[str]

    @cached_property
    def columns(self) -> list[str]:
        return list("".join(x) for x in zip(*self.rows))


@day.generator
def generator(input: str) -> Generator[Grid, None, None]:
    lines = [x.strip() for x in StringIO(input).readlines()]
    buf = []
    for line in lines:
        if line == "":
            yield Grid(buf)
            buf = []
        else:
            buf.append(line)
    if len(buf) > 0:
        yield Grid(buf)


def find_reflection(rows: list[str]) -> int | None:
    for l_idx, (left, right) in enumerate(sliding_window(rows, 2)):
        if left != right:
            continue
        # Found potential point of reflection, now lets see if
        # it is a true point of reflection
        l_reflection = rows[l_idx::-1]
        r_reflection = rows[l_idx + 1 : :]
        for l, r in zip(l_reflection, r_reflection):
            if l != r:
                break
        else:
            # Reflection found
            return l_idx + 1


@day.part1
def part1(grids: list[Grid]) -> int:
    rows = 0
    cols = 0
    for grid in grids:
        # find horizontal points of reflection
        rows += find_reflection(grid.rows) or 0
        cols += find_reflection(grid.columns) or 0

    return (rows * 100) + cols


def mirror_diff(left: str, right: str) -> int:
    """Returns the number of characters that differ in the mirror field"""
    return sum(int(l != r) for l, r in zip(left, right))


def find_reflection_with_smudge(rows: list[str]) -> int | None:
    # We're looking for a reflection point where only a single character
    # differs compared to a pure reflection
    for l_idx, (left, right) in enumerate(sliding_window(rows, 2)):
        l_reflection = rows[l_idx::-1]
        r_reflection = rows[l_idx + 1 : :]
        delta = sum(mirror_diff(x, y) for x, y in zip(l_reflection, r_reflection))

        if delta == 1:
            # Reflection found
            return l_idx + 1


@day.part2
def part2(grids: list[Grid]) -> int:
    rows = 0
    cols = 0
    for grid in grids:
        rows += find_reflection_with_smudge(grid.rows) or 0
        cols += find_reflection_with_smudge(grid.columns) or 0

    return (rows * 100) + cols


@day.test(405, 400)
def test():
    return """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""
