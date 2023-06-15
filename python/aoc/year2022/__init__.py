import dataclasses
import typing
from pathlib import Path
from os import path
import types
import importlib

# Default input files locatin
_DEFAULT_INPUT_FILES = Path(__file__).parent.joinpath("..", "..", "..", "input", "2022")


class Year:
    def __init__(self, year, input_files=_DEFAULT_INPUT_FILES.absolute()):
        self.year = year
        self.days = {}
        self.input_folder = input_files

    def day(self, name):
        day = Day(input=self.input_folder.joinpath(f"day{str(name)}.txt"))
        self.days[name] = day
        return day


class Day:
    def __init__(self, input=None):
        self._generator = lambda x: x
        self._part1 = None
        self._part2 = None
        self._input = input
        self._test_input = None
        self._test_part1 = None
        self._test_part2 = None

    def generator(self, other):
        self._generator = other

    def _run_generator(self, input):
        data = self._generator(input)
        if isinstance(data, types.GeneratorType):
            data = list(data)
        return data

    @property
    def has_part1(self):
        return self._part1 is not None

    def part1(self, other):
        self._part1 = other

    @property
    def has_part2(self):
        return self._part2 is not None

    def part2(self, other):
        self._part2 = other

    def run_part1(self):
        with open(self._input, "rt") as fobj:
            prepped = self._run_generator(fobj.read())
        return self._part1(prepped)

    def run_part2(self):
        with open(self._input, "rt") as fobj:
            prepped = self._run_generator(fobj.read())
        return self._part2(prepped)

    @property
    def has_test(self):
        return self._test_input is not None

    def test(self, part1_answer, part2_answer):
        self._test_part1 = part1_answer
        self._test_part2 = part2_answer

        def callback(input_gen):
            self._test_input = input_gen()
            return input_gen

        return callback

    def test_part1_successful(self):
        if self._test_part1 is not None:
            return (
                self._part1(self._run_generator(self._test_input)) == self._test_part1
            )

    def test_part2_successful(self):
        if self._test_part2 is not None:
            return (
                self._part2(self._run_generator(self._test_input)) == self._test_part2
            )


year2022 = Year("2022")

_submodules = [
    f"day{x}"
    for x in range(1, 24)
    if Path(__file__).parent.joinpath(f"day{x}.py").exists()
]
for _mod in _submodules:
    # Force import of each day to trigger day registration
    importlib.import_module(f".{_mod}", __package__)


__all__ = _submodules + [Day, Year, year2022]
