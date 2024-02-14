import collections
import itertools
from io import StringIO
from dataclasses import dataclass
from collections.abc import Sequence
from typing import Self


# Straight from the python itertools documentation
# https://docs.python.org/3/library/itertools.html
def sliding_window(iterable, n):
    "Collect data into overlapping fixed-length chunks or blocks."
    # sliding_window('ABCDEFG', 4) --> ABCD BCDE CDEF DEFG
    it = iter(iterable)
    window = collections.deque(itertools.islice(it, n - 1), maxlen=n)
    for x in it:
        window.append(x)
        yield tuple(window)


def split_by_blank_line(s: str) -> list[list[str]]:
    """Accepts a multi-line string and lists of strings grouped by the
    blank lines between them."""
    buf = []
    result = []
    for line in StringIO(s).readlines():
        line = line.strip()
        if len(line) == 0:
            result.append(buf)
            buf = []
        else:
            buf.append(line)
    if len(buf) > 0:
        result.append(buf)
    return result


@dataclass
class StrGrid2D:
    grid: Sequence[str]

    def rotate_clockwise(self) -> None:
        rotated = ("".join(x) for x in zip(*reversed(self.grid)))
        self.grid = tuple(rotated)

    def rotate_counterclockwise(self) -> None:
        rotated = reversed(tuple("".join(x) for x in zip(*self.grid)))
        self.grid = tuple(rotated)

    def reflect_across_identity(self) -> None:
        """
        Reflect values across the diagonal top left to bottom right.
        Note that this assumes you have an odd number of rows/cols.
        """
        reflected = ("".join(x) for x in zip(*self.grid))
        self.grid = tuple(reflected)
