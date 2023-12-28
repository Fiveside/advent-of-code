from . import year2023
from dataclasses import dataclass
from io import StringIO
from collections import Counter
import functools
from typing import Self

day = year2023.day(7)


@dataclass(frozen=True)
class Hand:
    cards: str
    bid: int


@day.generator
def generator(input: str) -> list[Hand]:
    for line in StringIO(input).readlines():
        cards, bid = line.strip().split()
        yield Hand(cards, int(bid))


@functools.total_ordering
class StandardRules:
    hand: Hand
    counter: Counter[str]
    most_common: list[tuple[str, int]]

    def __init__(self, hand: Hand):
        self.hand = hand
        self.counter = Counter(hand.cards)

    @functools.cached_property
    def most_common(self) -> list[tuple[str, int]]:
        return self.counter.most_common()

    def is_5oak(self) -> bool:
        return self.most_common[0][1] == 5

    def is_4oak(self) -> bool:
        return self.most_common[0][1] == 4

    def is_full_house(self) -> bool:
        return self.most_common[0][1] == 3 and self.most_common[1][1] == 2

    def is_3oak(self) -> bool:
        return self.most_common[0][1] == 3

    def is_2pair(self) -> bool:
        return self.most_common[0][1] == 2 and self.most_common[1][1] == 2

    def is_2oak(self) -> bool:
        return self.most_common[0][1] == 2

    @property
    def rank_ordering(self) -> str:
        return "AKQJT98765432"

    @functools.lru_cache
    def compare(self, other: Self) -> int:
        rules = [
            "is_5oak",
            "is_4oak",
            "is_full_house",
            "is_3oak",
            "is_2pair",
            "is_2oak",
        ]
        for rule in rules:
            lt, rt = getattr(self, rule)(), getattr(other, rule)()
            if lt or rt:
                if lt and rt:
                    # types are the same, compare ranks
                    return self.compare_ranks(other)
                else:
                    # types differ, declare winner
                    return -1 if lt else 1
        else:
            return 0

    def compare_ranks(self, other: Self) -> int:
        for l, r in zip(self.hand.cards, other.hand.cards):
            if l == r:
                continue
            return self.rank_ordering.index(l) - self.rank_ordering.index(r)
        else:
            return 0

    def __hash__(self):
        return hash(self.hand)

    def __lt__(self, other: Self) -> bool:
        return self.compare(other) < 0

    def __eq__(self, other: Self) -> bool:
        return self.compare(other) == 0


class JokerRule(StandardRules):
    jokers: int

    def __init__(self, hand: Hand):
        super().__init__(hand)

        self.jokers = self.counter.pop("J", 0)

    @functools.cached_property
    def most_common(self) -> list[tuple[str, int]]:
        mc = super().most_common
        if not mc:
            return [("J", self.jokers)]
        mc[0] = (mc[0][0], mc[0][1] + self.jokers)
        return mc

    @property
    def rank_ordering(self) -> str:
        return "AKQT98765432J"


@day.part1
def part1(games: list[Hand]) -> int:
    games.sort(key=StandardRules, reverse=True)
    score = sum(hand.bid * rank for rank, hand in enumerate(games, 1))
    return score


@day.part2
def part2(games: list[Hand]) -> int:
    games.sort(key=JokerRule, reverse=True)
    score = sum(hand.bid * rank for rank, hand in enumerate(games, 1))
    return score


@day.test(6440, 5905)
def test():
    return """32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"""
