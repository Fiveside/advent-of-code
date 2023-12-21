from . import year2023
from dataclasses import dataclass
from io import StringIO
from functools import reduce
import operator
import math

day = year2023.day(6)

# given maxtime:  7, dist = 9
# velocity * time_remaining = dist
# velocity = (maxtime - time_remaining)
# time_remaining*maxtime - time_remaining^2 = dist
# solve for time_remaining
# quadratic formula!
# time_remaining = (maxtime +- sqrt(maxtime^2 - 4*dist))/2
# Gives us the two possible points on the parabola where we can find
# where the winner began charging


def find_winner_charge_times(maxtime: int, distance: int) -> tuple[float, float]:
    sq = math.sqrt(maxtime**2 - (4 * distance))
    first = (maxtime + sq) / 2
    second = (maxtime - sq) / 2
    return (maxtime - first, maxtime - second)


def find_victory_range(time, distance) -> range:
    w_start, w_end = find_winner_charge_times(time, distance)
    start = math.ceil(w_start)
    end = math.floor(w_end)
    if w_start.is_integer():
        start += 1
    if w_end.is_integer():
        end -= 1
    return range(max(start, 1), min(end, time) + 1)


@dataclass
class Race:
    # Duration of race
    time: int

    # Distance to beat
    distance: int

    def count_victories(self) -> int:
        return len(find_victory_range(self.time, self.distance))


@dataclass
class Interpretations:
    raw: dict[str, list[str]]

    def as_multi_game(self) -> list[Race]:
        for time, dist in zip(self.raw["Time:"], self.raw["Distance:"]):
            yield Race(int(time), int(dist))

    def as_single_game(self) -> Race:
        time = "".join(self.raw["Time:"])
        dist = "".join(self.raw["Distance:"])
        return Race(int(time), int(dist))


@day.generator
def generator(input: str) -> Interpretations:
    by_prefix = dict()
    for line in StringIO(input).readlines():
        prefix, *rest = line.split()
        by_prefix[prefix] = [x.strip() for x in rest]

    return Interpretations(by_prefix)


@day.part1
def part1(input: Interpretations) -> int:
    return reduce(operator.mul, (x.count_victories() for x in input.as_multi_game()))


@day.part2
def part2(input: Interpretations) -> int:
    return input.as_single_game().count_victories()


@day.test(288, 71503)
def test():
    return """Time:      7  15   30
Distance:  9  40  200"""
