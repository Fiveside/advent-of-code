from . import year2025
from io import StringIO
from typing import Generator
from dataclasses import dataclass

day = year2025.day(1)


@day.generator
def sanitize(input: str) -> Generator[int, None, None]:
    for line in StringIO(input.strip()).readlines():
        line = line.strip()
        sign = 1
        if line.startswith("R"):
            pass
        elif line.startswith("L"):
            sign = -1
        else:
            raise ValueError(f"Unexpected line prefix: {line}")
        yield sign * int(line[1:])


def line_to_num(line: str) -> int:
    sanitized = [x for x in line if x.isdigit()]
    return int(sanitized[0] + sanitized[-1])


@dataclass
class SafeDial:
    cursor: int

    def __init__(self):
        self.cursor = 50

    def __call__(self, offset: int) -> bool:
        """returns True if we land on zero after applying this turn."""
        value = self.cursor + offset
        self.cursor = value % 100
        return self.cursor == 0


@day.part1
def part1(input: list[str]) -> int:
    dial = SafeDial()
    zeros = sum(dial(line_to_num(x)) for x in input)
    return zeros


@day.part2
def part2(input: list[str]) -> int:
    pass


@day.test(part1=3)
def test():
    return """L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"""


@day.test(part2=281)
def test2():
    return """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""
