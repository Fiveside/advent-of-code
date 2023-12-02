from . import year2023
from io import StringIO

day = year2023.day(1)


@day.generator
def sanitize(input: str) -> list[str]:
    for line in StringIO(input).readlines():
        yield line.strip()


def line_to_num(line: str) -> int:
    sanitized = [x for x in line if x.isdigit()]
    return int(sanitized[0] + sanitized[-1])


@day.part1
def part1(input: list[str]) -> int:
    return sum(line_to_num(x) for x in input)


NUMBERS = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
    # "zero": "0",
}


def numberize(line: str) -> str:
    for string, digit in NUMBERS.items():
        line = line.replace(string, digit)
    return line


@day.part2
def part2(input: list[str]) -> int:
    return sum(line_to_num(numberize(x)) for x in input)


@day.test(part1=142)
def test():
    return """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""


@day.test(part2=281)
def test2():
    return """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""
