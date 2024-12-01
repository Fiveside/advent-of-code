from io import StringIO
from typing import Generator, List
from . import year2024

day = year2024.day(1)


@day.generator
def generate(input: str) -> Generator[tuple[int, int], None, None]:
    for line in StringIO(input.strip()).readlines():
        l, r = line.split()
        yield (int(l.strip()), int(r.strip()))


@day.part1
def part1(rows: List[tuple[int, int]]) -> int:
    left, right = list(zip(*rows))
    left, right = sorted(left), sorted(right)
    return sum(abs(x - y) for x, y in zip(left, right))


@day.test(part1=11)
def test():
    return """3   4
4   3
2   5
1   3
3   9
3   3"""
