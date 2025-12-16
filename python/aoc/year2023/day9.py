from . import year2023
from dataclasses import dataclass
from io import StringIO
from typing import Generator
import functools
from .utils import sliding_window

day = year2023.day(9)


def extrapolate(values: list[int]) -> list[int]:
    return [y - x for x, y in sliding_window(values, 2)]


@dataclass
class Report:
    values: list[int]

    @functools.cached_property
    def extrapolations(self):
        exes = []
        current = self.values
        while True:
            current = extrapolate(current)
            exes.append(current)
            if all(x == 0 for x in current):
                break
        return exes

    def predict_next(self):
        delta = 0
        for ex in reversed(self.extrapolations):
            delta = ex[-1] + delta
        return self.values[-1] + delta

    def predict_prev(self):
        delta = 0
        for ex in reversed(self.extrapolations):
            delta = ex[0] - delta
        return self.values[0] - delta


@day.generator
def generator(input: str) -> Generator[Report, None, None]:
    for line in StringIO(input).readlines():
        yield Report(list(int(x) for x in line.strip().split()))


@day.part1
def part1(reports: list[Report]) -> int:
    return sum(x.predict_next() for x in reports)


@day.part2
def part2(reports: list[Report]) -> int:
    return sum(x.predict_prev() for x in reports)


@day.test(114, 2)
def test():
    return """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""
