from . import year2023
from dataclasses import dataclass
from io import StringIO

day = year2023.day(7)


@dataclass
class Hand:
    cards: str
    bid: int


@day.generator
def generator(input: str) -> list[Hand]:
    for line in StringIO(input).readlines():
        cards, bid = line.strip().split()
        yield Hand(cards, int(bid))


@day.part1
def part1(games: list[Hand]) -> int:
    pass


@day.test(6440)
def test():
    """32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"""
