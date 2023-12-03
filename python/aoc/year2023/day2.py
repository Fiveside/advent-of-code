from . import year2023
from dataclasses import dataclass
from io import StringIO

day = year2023.day(2)


@dataclass
class Play:
    red: int = 0
    green: int = 0
    blue: int = 0


@dataclass
class Game:
    id: int
    plays: list[Play]

    @property
    def max_red(self):
        return max(x.red for x in self.plays)

    @property
    def max_green(self):
        return max(x.green for x in self.plays)

    @property
    def max_blue(self):
        return max(x.blue for x in self.plays)


def parse_play(input: str) -> Play:
    params = (x.strip().split() for x in input.strip().split(","))
    return Play(**{element[1]: int(element[0]) for element in params})


@day.generator
def generator(input: str) -> list[Game]:
    for line in StringIO(input).readlines():
        left, right = line.split(":", maxsplit=1)
        id_str = left.strip().split()[-1]
        plays = [parse_play(x) for x in right.strip().split(";")]
        yield Game(id=int(id_str), plays=plays)


def is_valid_part1_game(game: Game) -> bool:
    return game.max_red <= 12 and game.max_green <= 13 and game.max_blue <= 14


@day.part1
def part1(input: list[Game]) -> int:
    return sum(x.id for x in input if is_valid_part1_game(x))


@day.part2
def part2(input: list[Game]) -> int:
    return sum(x.max_red * x.max_green * x.max_blue for x in input)


@day.test(8, 2286)
def test():
    return """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"""
