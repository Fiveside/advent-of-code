from collections.abc import Generator
from dataclasses import dataclass, field
from enum import Enum, auto
import re
from . import year2024

day = year2024.day(3)


class CommandType(Enum):
    Mul = auto()
    Command = auto()


@dataclass
class PositionEntry:
    type: CommandType = field(init=False)


@dataclass
class EnablerEntry(PositionEntry):
    enabled: bool

    def __post_init__(self):
        self.type = CommandType.Command


@dataclass
class MulEntry(PositionEntry):
    left: int
    right: int

    def __post_init__(self):
        self.type = CommandType.Mul


def find_muls(input: str) -> Generator[tuple[int, int, int], None, None]:
    for match in re.finditer(r"mul\((\d+),(\d+)\)", input):
        x, y = match.groups()
        yield int(x), int(y), match.start()


def find_dos(input: str) -> Generator[int, None, None]:
    for match in re.finditer(r"do\(\)", input):
        yield match.start()


def find_dont(input: str) -> Generator[int, None, None]:
    for match in re.finditer(r"don't\(\)", input):
        yield match.start()


@day.part1
def part1(input: str) -> int:
    return sum(x * y for x, y, _pos in find_muls(input))


@day.part2
def part2(input: str) -> int:
    entries = dict()
    for x, y, pos in find_muls(input):
        entries[pos] = MulEntry(x, y)

    for pos in find_dos(input):
        entries[pos] = EnablerEntry(True)

    for pos in find_dont(input):
        entries[pos] = EnablerEntry(False)

    sum = 0
    enabled = True
    for k in sorted(entries.keys()):
        val = entries[k]
        match val.type:
            case CommandType.Command:
                enabled = val.enabled
            case CommandType.Mul:
                if enabled:
                    sum = sum + (val.left * val.right)
    return sum


@day.test(part1=161)
def test():
    return """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"""


@day.test(part2=48)
def test2():
    return (
        """xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"""
    )
