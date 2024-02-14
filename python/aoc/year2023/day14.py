from collections.abc import Generator
from . import year2023
from .utils import split_by_blank_line, StrGrid2D
from dataclasses import dataclass
from typing import Self
from io import StringIO

day = year2023.day(14)


def tilt_str(s: str) -> str:
    # The currently available index that tilting this string would
    # park a boulder in.
    buf = list("." * len(s))
    avail = next(i for i, x in enumerate(s) if x != "#")

    for idx, c in enumerate(s):
        if c == "O":
            buf[avail] = "O"
            avail += 1
            while avail < len(buf) and buf[avail] != ".":
                avail += 1
        elif c == "#":
            buf[idx] = "#"
            avail = idx + 1
    return "".join(buf)


@dataclass
class Grid(StrGrid2D):
    pass

    def tilt_left(self) -> Self:
        """Tilts the board to the left, gathering
        all boulders as far left as they can go."""
        self.grid = tuple(tilt_str(x) for x in self.grid)

    def northern_load(self) -> int:
        load = 0
        for multiplier, line in enumerate(reversed(self.grid), 1):
            line_load = sum(1 if x == "O" else 0 for x in line)
            load += line_load * multiplier
        return load


@day.generator
def generator(input: str) -> Generator[Grid, None, None]:
    lines = []
    for line in StringIO(input).readlines():
        line = line.strip()
        if len(line) > 0:
            lines.append(line)

    return Grid(tuple(lines))


@day.part1
def part1(input: Grid) -> int:
    input.rotate_counterclockwise()
    input.tilt_left()
    input.rotate_clockwise()
    return input.northern_load()


@day.test(136)
def test():
    return """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."""
