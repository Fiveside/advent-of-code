from . import year2023
from typing import NamedTuple, Self
from io import StringIO
import itertools

day = year2023.day(11)


class Galaxy(NamedTuple):
    id: int
    x: int
    y: int

    def distance(self, other: Self) -> int:
        return abs(other.x - self.x) + abs(other.y - self.y)


@day.generator
def generator(input: str) -> list[Galaxy]:
    # First, iterate over rows and columns to figure out which ones are
    # expanding
    lines = [x.strip() for x in StringIO(input).readlines()]
    expanding_rows = []
    for line_no, line in enumerate(lines):
        if all(x == "." for x in line):
            expanding_rows.append(line_no)

    # Reorient our stuff here so we can count the columns
    expanding_cols = []
    for col_no, col in enumerate(zip(*lines)):
        if all(x == "." for x in col):
            expanding_cols.append(col_no)

    # Record galaxies
    galaxies = []
    delta_y = 0
    galaxy_id = 0
    for line_no, line in enumerate(lines):
        if line_no in expanding_rows:
            delta_y += 1
            continue
        delta_x = 0
        for col_no, cell in enumerate(line):
            if col_no in expanding_cols:
                delta_x += 1
                continue
            if cell == "#":
                galaxy_id += 1
                galaxies.append(Galaxy(galaxy_id, col_no + delta_x, line_no + delta_y))

    return galaxies


@day.part1
def part1(galaxies: list[Galaxy]) -> int:
    return sum(x.distance(y) for x, y in itertools.combinations(galaxies, 2))


@day.test(374)
def test():
    return """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""
