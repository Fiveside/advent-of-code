use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

struct RucksackItem(char);

impl RucksackItem {
    fn priority(&self) -> u32 {
        if self.0.is_ascii_uppercase() {
            27 + (self.0 as u32) - ('A' as u32)
        } else {
            1 + (self.0 as u32) - ('a' as u32)
        }
    }
}

struct RucksackCollection(String, String);

impl RucksackCollection {
    fn from_line_entry(line: &str) -> Self {
        let len = line.len();
        let (left, right) = line.split_at(len/2);
        Self(left.to_owned(), right.to_owned())
    }
}


#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<RucksackCollection> {
    input.lines().map(RucksackCollection::from_line_entry).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[RucksackCollection]) -> u32 {
    input.iter().map(|rc| {
        let left = rc.0.chars().collect::<HashSet<_>>();
        let right = rc.1.chars().collect::<HashSet<_>>();
        let common = left.intersection(&right).next().unwrap();
        RucksackItem(common.to_owned()).priority()
    }).fold(0, |x, y| x + y)
}

#[aoc(day3, part2)]
fn part2(input: &[RucksackCollection]) -> u32 {unimplemented!()}


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
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 157);
    }
}
