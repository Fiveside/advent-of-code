from collections.abc import Generator, Sequence
from . import year2023
from .utils import split_by_blank_line, StrGrid2D
from dataclasses import dataclass
from typing import Self
from io import StringIO

day = year2023.day(14)


def tilt_str(s: str | Sequence[str]) -> str:
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

    def tilt_left(self) -> None:
        """Tilts the board to the left, gathering
        all boulders as far left as they can go."""
        self.grid = tuple(tilt_str(x) for x in self.grid)

    def tilt_north(self) -> None:
        rotated = tuple(tilt_str(x) for x in zip(*self.grid))
        self.grid = tuple("".join(x) for x in zip(*rotated))

    def tilt_south(self) -> None:
        rotated = tuple(tilt_str(x[::-1]) for x in zip(*self.grid))
        self.grid = tuple("".join(x) for x in zip(*rotated))[::-1]

    def tilt_right(self) -> None:
        self.grid = tuple(tilt_str(x[::-1])[::-1] for x in self.grid)

    def northern_load(self) -> int:
        load = 0
        for multiplier, line in enumerate(reversed(self.grid), 1):
            line_load = sum(1 if x == "O" else 0 for x in line)
            load += line_load * multiplier
        return load

    def clone(self) -> Self:
        return Grid(self.grid)


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
    input.tilt_north()
    return input.northern_load()


def spin_cycle(grid: Grid) -> None:
    grid.tilt_north()
    grid.tilt_left()
    grid.tilt_south()
    grid.tilt_right()


@day.part2
def part2(grid: Grid) -> int:
    spins = 1_000_000_000
    cache = dict()
    order = list()
    for _ in range(spins):
        spin_cycle(grid)
        if grid.grid in cache:
            break
        else:
            cache[grid.grid] = len(order)
            order.append(grid.clone())
    # Measure the spin loop length
    # Calculate the number of spins until our goal
    # Modulus into the spin loop to figure out which spin we'll land on
    idx = cache[grid.grid]
    loop = order[idx:]
    remaining = spins - len(order)
    g = loop[remaining % len(loop)]
    return g.northern_load()

    # return grid.northern_load()


@day.test(136, 64)
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
