from . import year2023
from dataclasses import dataclass, astuple, field
import abc
from io import StringIO
import re
import operator
import functools
import enum
from typing import Self

day = year2023.day(19)


@dataclass
class Part:
    x: int
    m: int
    a: int
    s: int

    def rating(self):
        return sum(astuple(self))


class PartRatingRange:
    # [start, end) so start inclusive end exclusive
    start: int
    end: int

    @classmethod
    def default(cls):
        return cls(1, 4000)


class PartHueristic:
    x: PartRatingRange = field(init=False, default_factory=PartRatingRange.default)
    m: PartRatingRange = field(init=False, default_factory=PartRatingRange.default)
    a: PartRatingRange = field(init=False, default_factory=PartRatingRange.default)
    s: PartRatingRange = field(init=False, default_factory=PartRatingRange.default)


class RuleEvaluation(enum.Enum):
    ACCEPTED = enum.auto()
    REJECTED = enum.auto()
    CONTINUE = enum.auto()


@dataclass
class Rule(abc.ABC):
    @abc.abstractmethod
    def evaluate(self, part: "Part", rules: dict[str, "Workflow"]) -> RuleEvaluation:
        pass


@dataclass
class Workflow:
    name: str
    rules: list[Rule]

    def is_acceptable_part(
        self, part: Part, workflow_db: dict[str, Self]
    ) -> RuleEvaluation:
        for rule in self.rules:
            disposition = rule.evaluate(part, workflow_db)
            if disposition == RuleEvaluation.CONTINUE:
                continue
            return disposition
        assert False, f"exhausted all rules"


class AcceptRule(Rule):
    def evaluate(self, part: Part, rules: dict[str, Workflow]) -> RuleEvaluation:
        return RuleEvaluation.ACCEPTED


class RejectRule(Rule):
    def evaluate(self, part: Part, rules: dict[str, Workflow]) -> RuleEvaluation:
        return RuleEvaluation.REJECTED


@dataclass
class Conditional(Rule):
    prop: str
    operation: str
    val: int
    delegate: Rule

    def evaluate(self, part: Part, rules: dict[str, Workflow]) -> RuleEvaluation:
        part_val = getattr(part, self.prop)
        if self.operate(part_val, self.val):
            return self.delegate.evaluate(part, rules)
        else:
            return RuleEvaluation.CONTINUE

    @property
    def operate(self):
        if self.operation == "<":
            return operator.lt
        return operator.gt


@dataclass
class Jump(Rule):
    to: str

    def evaluate(self, part: Part, rules: dict[str, Workflow]) -> RuleEvaluation:
        return rules[self.to].is_acceptable_part(part, rules)


@dataclass
class Avalanche:
    parts: list[Part]
    workflows: dict[str, Workflow]

    @property
    def first_workflow(self):
        return self.workflows["in"]

    def is_acceptable_part(self, part: Part) -> bool:
        match self.first_workflow.is_acceptable_part(part, self.workflows):
            case RuleEvaluation.ACCEPTED:
                return True
            case RuleEvaluation.REJECTED:
                return False
        assert False, "unreachable"


def parse_str_rule(rule: str) -> Rule:
    if rule == "A":
        return AcceptRule()
    elif rule == "R":
        return RejectRule()
    else:
        return Jump(rule)


def parse_conditional(rule: str) -> Rule:
    res = re.match(r"(\w+)([\<\>])(\d+)\:(\w+)", rule)
    prop, oper, val, delegate = res.groups()
    return Conditional(
        prop=prop, operation=oper, val=int(val), delegate=parse_str_rule(delegate)
    )


@day.generator
def generator(input: str) -> Avalanche:
    workflows = []
    parts = []

    for line in StringIO(input).readlines():
        line = line.strip()
        if line == "":
            continue
        elif line[0].isalpha():
            flows = []
            name, *rule_strs = (
                x.group() for x in re.finditer(r"([\w\d\<\>\:]+)", line)
            )
            for rule in rule_strs:
                if rule.isalpha():
                    flows.append(parse_str_rule(rule))
                else:
                    flows.append(parse_conditional(rule))
            workflows.append(Workflow(name, flows))
        elif line[0] in r"{}":
            res = re.finditer(r"([xmas]\=\d+)", line)
            args = (x.group().split("=") for x in res)
            parts.append(Part(**{k: int(v) for k, v in args}))
        else:
            assert line.strip() == ""

    flows = {x.name: x for x in workflows}
    return Avalanche(parts=parts, workflows=flows)


@day.part1
def part1(avalanche: Avalanche) -> int:
    return sum(x.rating() for x in avalanche.parts if avalanche.is_acceptable_part(x))


@day.test(19114)
def test():
    return """px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"""
