from . import year2025
from io import StringIO
from typing import Generator
from dataclasses import dataclass
from enum import Enum, auto

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


class DialDisposition(Enum):
    NO_OVERFLOW = auto()
    OVERFLOW = auto()
    ENDS_ON_ZERO = auto()


@dataclass
class SafeDial:
    cursor: int

    def __init__(self):
        self.cursor = 50

    def ends_on_zero(self, offset: int) -> bool:
        """returns True if we land on zero after applying this turn."""
        value = self.cursor + offset
        self.cursor = value % 100
        return self.cursor == 0

    def passes_zero(self, offset: int) -> int:
        """Returns the number of times the dial passes zero including if it lands on zero."""
        start = self.cursor
        value = self.cursor + offset
        # Naieve implementation because my brain is cooked right now
        zeros = 0
        if value == 0:
            zeros = 1
        else:
            # Hacky shit
            if start == 0 and value < 0:
                zeros -= 1

            while value >= 100:
                value -= 100
                zeros += 1
            while value < 0:
                value += 100
                zeros += 1

        # if value == 100:
        #     value = 0
        #     zeros += 1
        # if value == 0:
        #     zeros += 1

        self.cursor = value
        return zeros


@day.part1
def part1(input: list[int]) -> int:
    dialer = SafeDial()
    zeros = sum(dialer.ends_on_zero(x) for x in input)
    return zeros


@day.part2
def part2(input: list[int]) -> int:
    dialer = SafeDial()
    ticks_past_zero = sum(dialer.passes_zero(x) for x in input)
    return ticks_past_zero


@day.test(part1=3, part2=6)
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
