from dataclasses import dataclass
from . import year2022
import string
import functools
import operator

day = year2022.day(3)


@dataclass
class Rucksack:
    first_compartment: str
    second_compartment: str

    def uniq(self):
        return set(self.first_compartment + self.second_compartment)


@day.generator
def generator(input: str) -> list[Rucksack]:
    for line in input.strip().split("\n"):
        line = line.strip()
        half = len(line) // 2
        yield Rucksack(line[:half], line[half:])


@day.part1
def part1(input: list[Rucksack]):
    acc = 0

    for sack in input:
        common = set(sack.first_compartment) & set(sack.second_compartment)
        assert len(common) == 1
        item = next(iter(common))
        acc += score(item)

    return acc


def score(char: str):
    return list(string.ascii_lowercase + string.ascii_uppercase).index(char) + 1


def partition(it, partition_size=3):
    while True:
        res = list()
        try:
            for _ in range(partition_size):
                res.append(next(it))
        except StopIteration as ex:
            if len(res) == 0:
                return
            raise ValueError("iterator is not divisible by partition size") from ex
        yield res


def group_score(input: list[Rucksack]):
    for things in partition(iter(input), 3):
        common = functools.reduce(operator.and_, (x.uniq() for x in things))
        assert len(common) == 1
        yield score(next(iter(common)))


@day.part2
def part2(input: list[Rucksack]):
    return sum(group_score(input))


@day.test(157, 70)
def test():
    return """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"""
