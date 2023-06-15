from . import year2022
import dataclasses
import io

day = year2022.day(1)


@dataclasses.dataclass
class Elf:
    calories: list[int] = dataclasses.field(default_factory=list)

    @property
    def total_calories(self):
        return sum(self.calories)


@day.generator
def generator(input: str) -> Elf:
    fobj = io.StringIO(input.strip())
    it = (x.strip() for x in fobj.readlines())
    elves = []

    elf = Elf()
    for line in it:
        if len(line) <= 0:
            elves.append(elf)
            elf = Elf()
        else:
            elf.calories.append(int(line))
    if len(elf.calories) > 0:
        elves.append(elf)

    return elves


@day.part1
def part1(input: list[Elf]) -> int:
    return max(x.total_calories for x in input)


@day.part2
def part2(input: list[Elf]) -> int:
    elves = sorted(input, key=lambda x: x.total_calories)
    return sum(x.total_calories for x in elves[-3:])


@day.test(24000, 45000)
def test():
    return """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"""
