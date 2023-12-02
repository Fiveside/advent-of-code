import types


class Year:
    def __init__(self, year, input_files):
        self.year = year
        self.days = {}
        self.input_folder = input_files

    def day(self, name):
        day = Day(input=self.input_folder.joinpath(f"day{str(name)}.txt"))
        self.days[name] = day
        return day


class Test:
    def __init__(self, input, part1=None, part2=None):
        self.input = input
        self.part1_expected = part1
        self.part2_expected = part2

    @property
    def has_part1(self):
        return self.part1_expected is not None

    @property
    def has_part2(self):
        return self.part2_expected is not None


class Day:
    def __init__(self, input=None):
        self._generator = lambda x: x
        self._part1 = None
        self._part2 = None
        self._input = input
        self._tests = list()
        # self._test_input = None
        # self._test_part1 = None
        # self._test_part2 = None

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

    def test(self, part1=None, part2=None):
        def callback(input_gen):
            self._tests.append(
                Test(
                    input=input_gen(),
                    part1=part1,
                    part2=part2,
                )
            )
            return input_gen

        return callback

    def test_all_part1(self):
        disposition = ""
        for test in self._tests:
            prepped = self._run_generator(test.input)
            if test.has_part1:
                if self._part1(prepped) == test.part1_expected:
                    disposition = disposition + "🟢"
                else:
                    disposition = disposition + "🔴"
            else:
                disposition = disposition + "⚪️"
        return disposition

    def test_all_part2(self):
        disposition = ""
        for test in self._tests:
            prepped = self._run_generator(test.input)
            if test.has_part2:
                if self._part2(prepped) == test.part2_expected:
                    disposition = disposition + "🟢"
                else:
                    disposition = disposition + "🔴"
            else:
                disposition = disposition + "⚪️"
        return disposition
