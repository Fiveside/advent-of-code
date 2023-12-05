from . import year2023
from io import StringIO
from dataclasses import dataclass
from collections import defaultdict

day = year2023.day(4)


@dataclass
class Game:
    id: int
    guesses: set[int]
    solutions: set[int]

    def count_wins(self):
        return len(self.guesses & self.solutions)


@day.generator
def generator(input: str) -> list[Game]:
    for line in StringIO(input).readlines():
        prefix, suffix = line.strip().split(":")
        id = int(prefix.strip().split(maxsplit=1)[1])
        guesses_buf, solutions_buf = suffix.strip().split("|")
        guesses = {int(x) for x in guesses_buf.strip().split()}
        solutions = {int(x) for x in solutions_buf.strip().split()}
        yield Game(id=id, guesses=guesses, solutions=solutions)


def score_part1(num_wins: int) -> int:
    if num_wins == 0:
        return 0

    num_wins -= 1
    score = 1
    while num_wins > 0:
        score = score * 2
        num_wins -= 1

    return score


@day.part1
def part1(input: list[Game]) -> int:
    return sum(score_part1(x.count_wins()) for x in input)


@day.part2
def part2(input: list[Game]) -> int:
    copies = defaultdict(lambda: 1)
    for id, game in enumerate(input):
        # Ensure that the current game is activated by the default dict
        copies[id]
        for x in range(0, game.count_wins()):
            copies[id + x + 1] += copies[id]

    return sum(copies.values())


@day.test(13, 30)
def test():
    return """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"""
