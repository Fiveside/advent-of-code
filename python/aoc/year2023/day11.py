from . import year2023
from typing import NamedTuple, Self
from io import StringIO
import itertools
from dataclasses import dataclass

day = year2023.day(11)


class Galaxy(NamedTuple):
    id: int
    x: int
    y: int

    def distance(self, other: Self) -> int:
        return abs(other.x - self.x) + abs(other.y - self.y)


@dataclass
class Universe:
    expanding_rows: list[int]
    expanding_cols: list[int]

    # These galaxies are not adjusted for universe expansion
    galaxies: list[Galaxy]

    def galaxies_with_expansion(self, multiplier=2):
        galaxies = []
        for galaxy in self.galaxies:
            dx = sum(1 for x in self.expanding_cols if x < galaxy.x) * (multiplier - 1)
            dy = sum(1 for x in self.expanding_rows if x < galaxy.y) * (multiplier - 1)
            galaxies.append(Galaxy(galaxy.id, galaxy.x + dx, galaxy.y + dy))
        return galaxies


@day.generator
def generator(input: str) -> Universe:
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
    galaxy_id = 0
    for line_no, line in enumerate(lines):
        for col_no, cell in enumerate(line):
            if cell == "#":
                galaxy_id += 1
                galaxies.append(Galaxy(galaxy_id, col_no, line_no))

    return Universe(expanding_rows, expanding_cols, galaxies)


@day.part1
def part1(universe: Universe) -> int:
    galaxies = universe.galaxies_with_expansion(2)
    return sum(x.distance(y) for x, y in itertools.combinations(galaxies, 2))


@day.part2
def part2(universe: Universe) -> int:
    galaxies = universe.galaxies_with_expansion(1_000_000)
    return sum(x.distance(y) for x, y in itertools.combinations(galaxies, 2))


@day.test(374, 82000210)
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
