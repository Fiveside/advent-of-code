from . import year2022
from dataclasses import dataclass
from enum import Enum

print("Imported!")

day = year2022.day(2)


class Play(Enum):
    ROCK = "A"
    PAPER = "B"
    SCISSORS = "C"

    def beats(self, other):
        return (self, other) in WINNING_SCENARIOS


class Disposition(Enum):
    LOSE = "X"
    TIE = "Y"
    WIN = "Z"

    def as_play(self):
        match self:
            case self.LOSE:
                return Play.ROCK
            case self.TIE:
                return Play.PAPER
            case self.WIN:
                return Play.SCISSORS

    def get_cause(self, other):
        """get play that would cause this disposition given the other play"""
        match self:
            case self.LOSE:
                return WIN_MAP[other]
            case self.TIE:
                return other
            case self.WIN:
                return REQUIRED_TO_WIN[other]


WIN_MAP = {
    Play.ROCK: Play.SCISSORS,
    Play.PAPER: Play.ROCK,
    Play.SCISSORS: Play.PAPER,
}

WINNING_SCENARIOS = list(WIN_MAP.items())
REQUIRED_TO_WIN = {y: x for x, y in WIN_MAP.items()}


MY_PLAY_SCORES = {
    Play.ROCK: 1,
    Play.PAPER: 2,
    Play.SCISSORS: 3,
}


@dataclass
class Round:
    elf_hand: Play
    counterplay: Play

    def disposition_score(self):
        if self.elf_hand.beats(self.counterplay):
            return 0
        elif self.counterplay.beats(self.elf_hand):
            return 6
        return 3

    def score(self):
        return MY_PLAY_SCORES[self.counterplay] + self.disposition_score()


@day.generator
def generator(input: str) -> list[tuple[Play, Disposition]]:
    for line in input.strip().split("\n"):
        l, r = line.strip().split()
        yield Play(l), Disposition(r)


@day.part1
def part1(input: list[tuple[Play, Disposition]]):
    rounds = (Round(l, r.as_play()) for l, r in input)
    return sum(x.score() for x in rounds)


@day.part2
def part2(input: list[tuple[Play, Disposition]]):
    rounds = (Round(l, r.get_cause(l)) for l, r in input)
    return sum(x.score() for x in rounds)


@day.test(15, 12)
def test():
    return """A Y
B X
C Z
"""
