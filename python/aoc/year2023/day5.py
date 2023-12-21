from . import year2023
from io import StringIO
import re
from dataclasses import dataclass, field
import itertools
from functools import reduce

day = year2023.day(5)


@dataclass
class RangeTranslationResult:
    remaining: list[range] = field(default_factory=list)
    translated: list[range] = field(default_factory=list)

    def __iter__(self):
        yield from (self.remaining, self.translated)

    def __getitem__(self, keys):
        return iter(getattr(self, k) for k in keys)


@dataclass
class RangeTranslationRule:
    from_id: int
    to_id: int
    count: int

    @classmethod
    def parse(cls, line):
        sections = line.strip().split()
        assert len(sections) == 3
        to_id, from_id, count = (int(x) for x in sections)
        return cls(from_id, to_id, count)

    def can_translate_id(self, id: int) -> bool:
        return id >= self.from_id and id < self.from_id + self.count

    def translate_id(self, id: int) -> int:
        assert self.can_translate_id(id)
        delta = id - self.from_id
        return self.to_id + delta

    def translate_range(self, r: range) -> RangeTranslationResult:
        # Cases:
        # 1: Rule encapsulates entire range, translate whole range
        # 2: Rule encapsulates beginning of range, translate partial range and
        #    return remaining range
        # 3: Rule encapsulates end of range, same deal
        # 4: Rule chunks out a portion of the range, return 2 partial remainings
        #    and one partial translation
        # 5: Rule matches nothing, just return ourself as is.

        # Unchecked range translation, makes math easier.
        t = range(r.start + self.from_delta, r.stop + self.from_delta)

        if self.from_range.start <= r.start and self.from_range.stop >= r.stop:
            # Case 1
            return RangeTranslationResult(translated=[t])
        elif max(r.start, self.from_range.start) > min(r.stop, self.from_range.stop):
            # Case 5
            return RangeTranslationResult(remaining=[range(r.start, r.stop)])
        elif self.from_range.start <= r.start and self.from_range.stop < r.stop:
            # Case 2
            return RangeTranslationResult(
                translated=[range(t.start, self.to_range.stop)],
                remaining=[range(self.from_range.stop, r.stop)],
            )
        elif self.from_range.start > r.start and self.from_range.stop >= r.stop:
            # Case 3
            return RangeTranslationResult(
                translated=[range(self.to_range.start, t.stop)],
                remaining=[range(r.start, self.from_range.start)],
            )
        elif self.from_range.start > r.start and self.from_range.stop < r.stop:
            # Case 4
            return RangeTranslationResult(
                translated=[self.to_range],
                remaining=[
                    range(r.start, self.from_range.start),
                    range(self.from_range.stop, r.stop),
                ],
            )
        else:
            # Should be inaccessible.
            raise NotImplementedError()

    @property
    def from_range(self):
        return range(self.from_id, self.from_id + self.count)

    @property
    def to_range(self):
        return range(self.to_id, self.to_id + self.count)

    @property
    def from_delta(self):
        return self.to_id - self.from_id


@dataclass
class SeedMapper:
    from_type: str
    to_type: str
    rules: list[RangeTranslationRule] = field(default_factory=list)

    def translate(self, id: int) -> int:
        for rule in self.rules:
            if rule.can_translate_id(id):
                return rule.translate_id(id)
        else:
            return id

    def translate_range(self, r: range) -> list[range]:
        def reducer(
            acc: RangeTranslationResult, rule: RangeTranslationRule
        ) -> RangeTranslationResult:
            remaining, translated = acc
            new_remaining = []
            for item in remaining:
                res = rule.translate_range(item)
                new_remaining.extend(res.remaining)
                translated.extend(res.translated)
            return RangeTranslationResult(
                remaining=new_remaining, translated=translated
            )

        rtr = reduce(reducer, self.rules, RangeTranslationResult(remaining=[r]))
        return rtr.remaining + rtr.translated


@dataclass
class Almanac:
    seeds: list[int]
    mappers: dict[str, SeedMapper]

    def translate_seed_to(self, id: int, dest_type: str) -> int:
        current_type = "seed"
        while True:
            if current_type == dest_type:
                return id
            mapper = self.mappers[current_type]
            id = mapper.translate(id)
            current_type = mapper.to_type

    def translate_seed_range_to(self, r: range, dest_type: str) -> list[range]:
        current_type = "seed"
        ranges = [r]
        while True:
            if current_type == dest_type:
                return list(ranges)
            mapper = self.mappers[current_type]

            ranges = list(
                itertools.chain.from_iterable(mapper.translate_range(x) for x in ranges)
            )

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
            current_mapper = SeedMapper(*match.groups())
            mappers.append(current_mapper)
        elif line.strip() == "":
            current_mapper = None
        else:
            current_mapper.rules.append(RangeTranslationRule.parse(line.strip()))

    return Almanac(seeds, {x.from_type: x for x in mappers})


def parse_seedline(line: str) -> set[int]:
    assert line.startswith("seeds: ")
    return [int(x.group()) for x in re.finditer(r"\d+", line)]


@day.part1
def part1(almanac: Almanac) -> int:
    locations = (
        almanac.translate_seed_to(seed_id, "location") for seed_id in almanac.seeds
    )
    return min(locations)


@day.part2
def part2(almanac: Almanac) -> int:
    seeds = []
    for start, count in itertools.batched(almanac.seeds, 2):
        seeds.append(range(start, start + count))

    locations = itertools.chain.from_iterable(
        almanac.translate_seed_range_to(r, "location") for r in seeds
    )

    return min(x.start for x in locations)


@day.test(35, 46)
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
