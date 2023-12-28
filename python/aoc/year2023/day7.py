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
        self.counter = Counter(hand)

    @functools.cached_property
    def most_common(self):
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


def without_jokers(hand_meta: Counter) -> tuple[Counter, int]:
    jokers = hand_meta["J"]
    new_meta = Counter({k: v for k, v in hand_meta.items() if k != "J"})
    return new_meta, jokers


def is_5oak(hand_meta: Counter, enable_jokers=False) -> bool:
    """Is 5 of a kind?"""
    if not enable_jokers:
        return hand_meta.most_common(1)[0][1] == 5
    meta, jokers = without_jokers(hand_meta)
    if meta == dict():
        # This is an awful place to put this
        # Handles the case where your hand is 'JJJJJ'
        # plz2refactor
        return True
    return meta.most_common(1)[0][1] + jokers == 5


def is_4oak(hand_meta: Counter, enable_jokers=False) -> bool:
    """Is 4 of a kind?"""
    if not enable_jokers:
        return hand_meta.most_common(1)[0][1] == 4
    meta, jokers = without_jokers(hand_meta)
    return meta.most_common(1)[0][1] + jokers == 4


def is_full_house(hand_meta: Counter, enable_jokers=False) -> bool:
    """Do we have a full house?"""
    if not enable_jokers:
        numbers = [x[1] for x in hand_meta.most_common()]
        return numbers == [3, 2]
    meta, jokers = without_jokers(hand_meta)
    numbers = [x[1] for x in meta.most_common()]
    return len(numbers) == 2 and sum(numbers) + jokers == 5


def is_3oak(hand_meta: Counter, enable_jokers=False) -> bool:
    """Is 3 of a kind and no full house?"""
    meta, jokers = without_jokers(hand_meta)
    if enable_jokers and jokers > 0:
        return meta.most_common(1)[0][1] + jokers == 3

    numbers = [x[1] for x in hand_meta.most_common()]
    return numbers == [3, 1, 1]


def is_two_pair(hand_meta: Counter, enable_jokers=False) -> bool:
    """Is 2 pairs?"""
    meta, jokers = without_jokers(hand_meta)
    if enable_jokers and jokers > 0:
        toptwo = sum(x[1] for x in meta.most_common(2))
        return toptwo + jokers == 4
    numbers = [x[1] for x in hand_meta.most_common()]
    return numbers == [2, 2, 1]


def is_2oak(hand_meta: Counter, enable_jokers=False) -> bool:
    """Is 2 of a kind and nothing else?"""
    meta, jokers = without_jokers(hand_meta)
    if enable_jokers and jokers == 1:
        return [x[1] for x in meta.most_common()] == [1, 1, 1, 1]
    numbers = [x[1] for x in hand_meta.most_common()]
    return numbers == [2, 1, 1, 1]


def hand_type_comparator(l: str, r: str, enable_jokers=False) -> int:
    lc = Counter(l)
    rc = Counter(r)

    tests = [
        is_5oak,
        is_4oak,
        is_full_house,
        is_3oak,
        is_two_pair,
        is_2oak,
    ]

    for test in tests:
        lt, rt = test(lc, enable_jokers), test(rc, enable_jokers)
        if lt or rt:
            if lt and rt:
                # Hands the same, search by first card rank
                return hand_rank_comparator(l, r, enable_jokers)
            else:
                # Hands differ, one is superior
                return -1 if lt else 1
    else:
        # Both are high card.
        return hand_rank_comparator(l, r, enable_jokers)


def hand_rank_comparator(l: str, r: str, enable_jokers=False) -> int:
    # Victory order
    if enable_jokers:
        order = "AKQT98765432J"
    else:
        order = "AKQJT98765432"
    for lc, rc in zip(l, r):
        if lc == rc:
            continue
        return order.index(lc) - order.index(rc)
    else:
        return 0


@day.part1
def part1(games: list[Hand]) -> int:
    key = functools.cmp_to_key(lambda l, r: hand_type_comparator(l.cards, r.cards))
    # games.sort(key=key, reverse=True)
    games.sort(key=StandardRules, reverse=True)
    score = sum(hand.bid * rank for rank, hand in enumerate(games, 1))
    return score


@day.part2
def part2(games: list[Hand]) -> int:
    key = functools.cmp_to_key(
        lambda l, r: hand_type_comparator(l.cards, r.cards, enable_jokers=True)
    )
    games.sort(key=key, reverse=True)
    score = sum(hand.bid * rank for rank, hand in enumerate(games, 1))
    return score


@day.test(6440, 5905)
def test():
    return """32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"""
