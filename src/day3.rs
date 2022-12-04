use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

fn priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        27 + (item as u32) - ('A' as u32)
    } else {
        1 + (item as u32) - ('a' as u32)
    }
}

struct RucksackPair(String, String);

impl RucksackPair {
    fn from_line_entry(line: &str) -> Self {
        let len = line.len();
        let (left, right) = line.split_at(len/2);
        Self(left.to_owned(), right.to_owned())
    }
}

struct ElfGroup(String, String, String);

impl ElfGroup {
    fn find_badge(&self) -> char {
        let left = self.0.chars().collect::<HashSet<char>>();
        let center = self.1.chars().collect::<HashSet<char>>();
        let right = self.2.chars().collect::<HashSet<char>>();
        *left.intersection(&center).filter(|&entry| right.contains(entry)).next().unwrap()
    }
}


#[aoc_generator(day3, part1)]
fn generator_part1(input: &str) -> Vec<RucksackPair> {
    input.lines().map(RucksackPair::from_line_entry).collect()
}

#[aoc_generator(day3, part2)]
fn generator_part2(input: &str) -> Vec<ElfGroup> {
    input.lines().tuples::<(_, _, _)>().map(|(l1, l2, l3)| ElfGroup(l1.to_owned(), l2.to_owned(), l3.to_owned())).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[RucksackPair]) -> u32 {
    input.iter().map(|rc| {
        let left = rc.0.chars().collect::<HashSet<_>>();
        let right = rc.1.chars().collect::<HashSet<_>>();
        let common = left.intersection(&right).next().unwrap();
        priority(common.to_owned())
    }).fold(0, |x, y| x + y)
}

#[aoc(day3, part2)]
fn part2(input: &[ElfGroup]) -> u32 {
    input.iter().map(|x| priority(x.find_badge())).sum()
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        let data = generator_part1(INPUT_TEXT);
        assert_eq!(part1(&data), 157);
    }

    #[test]
    fn test_part2() {
        let data = generator_part2(INPUT_TEXT);
        assert_eq!(part2(&data), 70);
    }
}
