from dataclasses import dataclass
from . import year2022
from collections import namedtuple
from typing import Self

day = year2022.day(4)


@dataclass
class Range:
    start: int
    end: int

    @property
    def range(self):
        return range(self.start, self.end + 1)

    def fully_contains(self, other: Self):
        return self.start <= other.start and self.end >= other.end

    def overlaps(self, other: Self):
        return self.start <= other.end and other.start <= self.end


Pair = namedtuple("ElfPair", ["l", "r"])


@day.generator
def generator(input: str):
    for line in input.strip().split("\n"):
        raw_left, raw_right = line.strip().split(",")
        yield Pair(
            Range(*(int(x) for x in raw_left.split("-"))),
            Range(*(int(x) for x in raw_right.split("-"))),
        )


@day.part1
def part1(input: list[Pair]):
    contains = [
        Pair(l, r) for l, r in input if l.fully_contains(r) or r.fully_contains(l)
    ]
    return len(contains)


@day.part2
def part2(input: list[Pair]):
    laps = [Pair(l, r) for l, r in input if l.overlaps(r)]
    return len(laps)


@day.test(2, 4)
def test_data():
    return """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""
