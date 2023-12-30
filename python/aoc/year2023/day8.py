from . import year2023
from dataclasses import dataclass, field
import re
from io import StringIO
import itertools
import operator
from typing import Callable, Generator
from collections.abc import Iterator
import math

day = year2023.day(8)


@dataclass(frozen=True)
class Node:
    name: str
    left: str
    right: str


@dataclass
class Network:
    instructions: str
    nodes: dict[str, Node]

    def instructions_as_getters(self) -> Generator[Callable[[Node], str], None, None]:
        for path in self.instructions:
            if path == "L":
                yield operator.attrgetter("left")
            else:
                yield operator.attrgetter("right")


def parse_node(nodestr: str) -> Node:
    res = re.match(r"(\w+) = \((\w+), (\w+)\)", nodestr)
    return Node(*res.groups())


@day.generator
def generator(input: str) -> Network:
    lines = StringIO(input).readlines()
    it = iter(lines)
    instructions = next(it).strip()

    nodes = []
    for line in it:
        line = line.strip()
        if len(line) == 0:
            continue
        nodes.append(parse_node(line))

    return Network(instructions, {x.name: x for x in nodes})


@day.part1
def part1(net: Network) -> int:
    node = net.nodes["AAA"]
    for steps, get in enumerate(itertools.cycle(net.instructions_as_getters())):
        if node.name == "ZZZ":
            return steps
        node = net.nodes[get(node)]


@dataclass
class Journey:
    name: str
    loop_length: int

    current: int = field(init=False, default=0)
    cycles: int = field(init=False, default=0)

    def step(self):
        self.cycles += 1
        self.current = (self.loop_length * self.cycles) + self.initial
        return self.current


def steps_to_z(node: Node, net: Network, ops: Iterator[Callable[[Node], str]]) -> int:
    for step, op in enumerate(ops, 1):
        node = net.nodes[op(node)]
        if node.name.endswith("Z"):
            return step


@day.part2
def part2(net: Network) -> int:
    nodes = [x for x in net.nodes.values() if x.name.endswith("A")]

    # An analysis of the input data reveals the following:
    # * Each start position is a member of the loop.  That is to say there is
    #   no initial journey needed to be taken before we arrive at the loop
    # * Many loops hit nodes multiple times before looping
    # * Tests do not loop, but instead hit the void afterward

    journeys = []
    for node in nodes:
        ops = itertools.cycle(net.instructions_as_getters())
        loop_length = steps_to_z(node, net, ops)
        journeys.append(Journey(node.name, loop_length))

    # An analysis of the input data reveals that our assumptions about journey
    # are not quite true:  When z loops back, it always loops back to the
    # beginning.  There is no initial stretch required to reach the loop.
    # Thus we can simplify this to an LCM operation
    return math.lcm(*(x.loop_length for x in journeys))


@day.test(2, 2)
def test():
    return """RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"""


@day.test(6, 6)
def test2():
    return """LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"""


@day.test(part2=6)
def test3():
    return """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"""
