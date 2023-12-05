from . import year2023
from io import StringIO
import re
from dataclasses import dataclass, field

day = year2023.day(5)


@dataclass
class MapRange:
    from_id: int
    to_id: int
    count: int

    @classmethod
    def parse(cls, line):
        sections = line.strip().split()
        assert len(sections) == 3
        to_id, from_id, count = (int(x) for x in sections)
        return cls(from_id, to_id, count)

    def can_translate(self, id: int) -> bool:
        return id >= self.from_id and id < self.from_id + self.count

    def translate(self, id: int) -> int:
        assert self.can_translate(id)
        delta = id - self.from_id
        return self.to_id + delta


@dataclass
class NumberMapper:
    from_type: str
    to_type: str
    rules: list[MapRange] = field(default_factory=list)

    def translate(self, id: int) -> int:
        for rule in self.rules:
            if rule.can_translate(id):
                return rule.translate(id)
        else:
            return id


@dataclass
class Almanac:
    seeds: set[int]
    mappers: dict[str, NumberMapper]

    def translate_seed_to(self, id: int, dest_type: str) -> int:
        current_type = "seed"
        while True:
            if current_type == dest_type:
                return id
            mapper = self.mappers[current_type]
            id = mapper.translate(id)
            current_type = mapper.to_type


@day.generator
def generator(input: str) -> Almanac:
    lines = StringIO(input).readlines()
    seedline = lines.pop(0)
    lines.pop(0)  # empty line
    seeds = parse_seedline(seedline)

    mappers = []
    current_mapper = None
    for line in lines:
        if line.strip().endswith("map:"):
            match = re.match(r"(\w+)-to-(\w+)\s+map:", line.strip())
            current_mapper = NumberMapper(*match.groups())
            mappers.append(current_mapper)
        elif line.strip() == "":
            current_mapper = None
        else:
            current_mapper.rules.append(MapRange.parse(line.strip()))

    return Almanac(seeds, {x.from_type: x for x in mappers})


def parse_seedline(line: str) -> set[int]:
    assert line.startswith("seeds: ")
    return {int(x.group()) for x in re.finditer(r"\d+", line)}


@day.part1
def part1(almanac: Almanac) -> int:
    locations = (
        almanac.translate_seed_to(seed_id, "location") for seed_id in almanac.seeds
    )
    return min(locations)


@day.test(35)
def test():
    return """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""
