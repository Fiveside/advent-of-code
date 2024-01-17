from . import year2023
from dataclasses import dataclass
from io import StringIO
from typing import Generator
import collections
import itertools
import functools

day = year2023.day(12)


@dataclass
class Row:
    groups: tuple[int]
    positions: str


@day.generator
def generator(input: str) -> Generator[Row, None, None]:
    for line in StringIO(input).readlines():
        positions, raw_groups = line.strip().split()
        groups = tuple(int(x) for x in raw_groups.split(","))
        yield Row(groups, positions)


def sliding_str_window_with_bookends(
    iterable: str, n: int
) -> Generator[tuple[str, str, str], None, None]:
    """
    Collect data into overlapping fixed-length chunks or blocks.
    Yields
    """
    # sliding_window('ABCDEFG', 4) --> ABCD BCDE CDEF DEFG
    it = iter(iterable)
    window = collections.deque(itertools.islice(it, n - 1), maxlen=n)
    for idx, x in enumerate(it):
        window.append(x)
        prefix = iterable[:idx]
        suffix = iterable[idx + n :]
        yield (prefix, "".join(window), suffix)


@functools.cache
def fit_row(groups: list[int], positions: str) -> int:
    group, *remaining_groups = groups
    total = 0

    for prefix, block, suffix in sliding_str_window_with_bookends(positions, group):
        # # If our block size isn't large enough to fit the group, that's because
        # # we've run out of string to fit our groups
        # if len(block) < group:
        #     break

        # The block is only usable if all elements are #? and if the element after
        # the block is .? or end of string
        if not all(x in r"#?" for x in block):
            continue

        if len(suffix) > 0:
            if suffix[0] not in r".?":
                continue

        # The block is only valid if we have not skipped any # elements
        # to get here
        if r"#" in prefix:
            break

        # We have successfuly passed all the criteria at this point and fit the block
        if len(remaining_groups) == 0:
            # If this is the last block we are trying to fit, then make sure we have
            # no # in successive positions
            if r"#" not in suffix:
                total += 1
            # yield [len(prefix)]
        else:
            # If its not the last block, then recurse and adjust true offsets:
            total += fit_row(tuple(remaining_groups), suffix[1:])
            # for success in fit_row(remaining_groups, suffix[1:]):
            #     yield [len(prefix)] + [x + len(prefix) for x in success]

    return total


@day.part1
def part1(rows: list[Row]) -> int:
    total = 0
    for row in rows:
        this_row = fit_row(row.groups, row.positions)
        total += this_row
    return total


@day.part2
def part2(rows: list[Row]) -> int:
    total = 0
    for row in rows:
        groups = row.groups * 5
        positions = "?".join(
            (row.positions, row.positions, row.positions, row.positions, row.positions)
        )
        this_row = fit_row(groups, positions)
        total += this_row
    return total


@day.test(21, 525152)
def test():
    return """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""
