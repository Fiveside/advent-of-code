from collections.abc import Generator
import re
from . import year2024

day = year2024.day(3)


def find_muls(input: str) -> Generator[tuple[int, int], None, None]:
    for match in re.finditer(r"mul\((\d+),(\d+)\)", input):
        x, y = match.groups()
        yield int(x), int(y)


@day.part1
def part1(input: str) -> int:
    return sum(x * y for x, y in find_muls(input))


@day.test(part1=161)
def test():
    return """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"""
