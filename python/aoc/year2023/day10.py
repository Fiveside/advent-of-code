from . import year2023
from io import StringIO
from dataclasses import dataclass, field
from enum import Enum, auto
from collections.abc import Generator
from typing import Self
import pprint
import math

day = year2023.day(10)


class Connection(Enum):
    north = auto()
    south = auto()
    east = auto()
    west = auto()

    @property
    def reverse(self):
        match self:
            case Connection.north:
                return Connection.south
            case Connection.south:
                return Connection.north
            case Connection.west:
                return Connection.east
            case Connection.east:
                return Connection.west


@dataclass
class Square:
    declared_connections: list[Connection]
    connections: set[Self] = field(default_factory=set)

    def __hash__(self):
        return id(self)


@dataclass
class Grid:
    grid: list[list[Square]]
    start: Square


def char_to_connection(char: str) -> list[Connection]:
    match char:
        case "|":
            return [Connection.north, Connection.south]
        case "-":
            return [Connection.east, Connection.west]
        case "F":
            return [Connection.south, Connection.east]
        case "7":
            return [Connection.south, Connection.west]
        case "J":
            return [Connection.north, Connection.west]
        case "L":
            return [Connection.north, Connection.east]
    assert char in "S.", f"Got unknown squre: {char}"
    return []


def parse_line(line: str) -> tuple[list[Square], Square | None]:
    start = None
    squares = list()
    for char in line:
        sq = Square(declared_connections=char_to_connection(char))
        if char == "S":
            start = sq
        squares.append(sq)
    return squares, start


@day.generator
def generator(input: str) -> Grid:
    grid: list[list[Square]] = list()
    start: Square | None = None

    for line in StringIO(input).readlines():
        line = line.strip()
        if not line:
            continue
        squares, maybe_start = parse_line(line)
        if maybe_start is not None:
            start = maybe_start
        grid.append(squares)

    # We still don't know what squares start connects to
    # We can also optimize future logic by adding direct
    # references to neighbors to each square, which will
    # address the unknown start

    for line_no, line in enumerate(grid):
        for col_no, square in enumerate(line):
            if square is start:
                continue
            for dc in square.declared_connections:
                remote = None
                match dc:
                    case Connection.north:
                        if line_no > 0:
                            remote = grid[line_no - 1][col_no]
                    case Connection.south:
                        if line_no < len(grid) - 1:
                            remote = grid[line_no + 1][col_no]
                    case Connection.west:
                        if col_no > 0:
                            remote = grid[line_no][col_no - 1]
                    case Connection.east:
                        if col_no < len(line) - 1:
                            remote = grid[line_no][col_no + 1]
                if remote is not None:
                    # Only add the connections if both parties agree or if one is the start
                    if remote is start or dc.reverse in remote.declared_connections:
                        remote.connections.add(square)
                        square.connections.add(remote)

    return Grid(grid=grid, start=start)


def walk_grid(start: Square) -> Generator[Square, None, None]:
    last = start
    current = next(iter(start.connections))
    while current is not start:
        yield current
        next_square = next(x for x in current.connections if x is not last)
        last = current
        current = next_square


@day.part1
def part1(grid: Grid) -> int:
    for stepnum, _square in enumerate(walk_grid(grid.start), start=1):
        pass
    return math.ceil(stepnum / 2)


@day.test(4)
def test():
    return """.....
.S-7.
.|.|.
.L-J.
....."""


@day.test(8)
def test2():
    return """7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"""
