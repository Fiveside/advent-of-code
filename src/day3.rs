use aoc_runner_derive::{aoc, aoc_generator};

struct Entry(char);

impl Entry {
    fn priority(&self) -> u32 {
        if self.0.is_ascii_uppercase() {
            27 + (self.0 as u32) - ('A' as u32)
        } else {
            1 + (self.0 as u32) - ('a' as u32)
        }
    }
}


#[aoc_generator(day3)]
fn generator(input: &str) -> u32 {
    input.lines().
}

#[aoc(day3, part1)]
fn part1(input: &u32) -> u32 {unimplemented!()}

#[aoc(day3, part2)]
fn part2(input: &u32) -> u32 {unimplemented!()}


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
