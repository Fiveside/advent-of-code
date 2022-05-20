use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn parse_chars(input: &str) -> Vec<String> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn is_unique_digit(input: &str) -> bool {
    match input.len() {
        2..=4 => true,
        7 => true,
        _ => false,
    }
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.rsplit_once('|').unwrap();
            Entry {
                patterns: parse_chars(left),
                output: parse_chars(right),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Entry]) -> i32 {
    input
        .iter()
        .map(|entry| entry.output.iter().filter(|s| is_unique_digit(s)).count())
        .sum::<usize>() as i32
}

// identifying 3
// find entries with 5 chars, they are 2, 3, and 5
// find entry with 2 chars, that's 1.
// find common 3 points in 5 chars, that's top mid bot
// find 5 char entry that has top mid bot + 1 chars, that's 3.

// Compare 1 and 7, the char that 7 has that 1 doesn't is A

// Collect 6 char entries, that's 0, 6, and 9.
// Compare chars with 1.  The char that does not include 1 is 6, and the missing
// letter is C.

// Compare 2, 3, 5. The one not containing C is 5.  The letter that 5 has that
// 1 also has is F
// The letter that 2 has that 5 does not is E.
// The letter that 3 is missing besides E is B.

// At this point we have identified A, B, C, E, and F.
// Take 4, the letter 4 has that we have not identified yet is D.
// Take 3.  The letter that 3 has that we have not identified is G.

#[derive(Debug)]
struct Mapping {
    zero: String,
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
    six: String,
    seven: String,
    eight: String,
    nine: String,
}

impl Mapping {
    fn new(a: char, b: char, c: char, d: char, e: char, f: char, g: char) -> Self {
        let mut zero = [a, b, c, e, f, g];
        let mut one = [c, f];
        let mut two = [a, c, d, e, g];
        let mut three = [a, c, d, f, g];
        let mut four = [b, c, d, f];
        let mut five = [a, b, d, f, g];
        let mut six = [a, b, d, e, f, g];
        let mut seven = [a, c, f];
        let mut eight = [a, b, c, d, e, f, g];
        let mut nine = [a, b, c, d, f, g];

        zero.sort();
        one.sort();
        two.sort();
        three.sort();
        four.sort();
        five.sort();
        six.sort();
        seven.sort();
        eight.sort();
        nine.sort();

        Self {
            zero: zero.into_iter().collect(),
            one: one.into_iter().collect(),
            two: two.into_iter().collect(),
            three: three.into_iter().collect(),
            four: four.into_iter().collect(),
            five: five.into_iter().collect(),
            six: six.into_iter().collect(),
            seven: seven.into_iter().collect(),
            eight: eight.into_iter().collect(),
            nine: nine.into_iter().collect(),
        }
    }

    fn translate(&self, input: &str) -> u64 {
        let i_sorted: String = input.chars().sorted().collect();
        match i_sorted {
            _ if i_sorted == self.zero => 0,
            _ if i_sorted == self.one => 1,
            _ if i_sorted == self.two => 2,
            _ if i_sorted == self.three => 3,
            _ if i_sorted == self.four => 4,
            _ if i_sorted == self.five => 5,
            _ if i_sorted == self.six => 6,
            _ if i_sorted == self.seven => 7,
            _ if i_sorted == self.eight => 8,
            _ if i_sorted == self.nine => 9,
            _ => unreachable!(),
        }
    }
}

fn find_exactly_len(input: &Entry, len: usize) -> &str {
    input
        .patterns
        .iter()
        .map(|x| x.as_str())
        .filter(|x| x.len() == len)
        .next()
        .unwrap()
}

fn find_potential_len_set(input: &Entry, len: usize) -> Vec<BTreeSet<char>> {
    input
        .patterns
        .iter()
        .filter(|x| x.len() == len)
        .map(|x| x.as_str().chars().collect())
        .collect()
}

fn derive_mapping(input: &Entry) -> Mapping {
    // Identify concrete number patterns
    let exactly_1 = find_exactly_len(input, 2);
    let exactly_1_set: BTreeSet<char> = exactly_1.chars().collect();
    let exactly_7 = find_exactly_len(input, 3);
    let exactly_4 = find_exactly_len(input, 4);
    let exactly_8_set: BTreeSet<char> = find_exactly_len(input, 7).chars().collect();

    // Identify 3.
    let potential_235 = find_potential_len_set(input, 5);
    let potential_top_mid_bot = potential_235
        .iter()
        .cloned()
        .reduce(|acc, r| acc.intersection(&r).cloned().collect())
        .unwrap();

    let exactly_3: String = potential_top_mid_bot
        .iter()
        .copied()
        .chain(exactly_1.chars())
        .collect();
    let exactly_3_set: BTreeSet<char> = exactly_3.chars().collect();

    // identify A
    let exactly_a = exactly_7
        .chars()
        .collect::<BTreeSet<char>>()
        .difference(&exactly_1_set)
        .next()
        .unwrap()
        .to_owned();

    let potential_069 = find_potential_len_set(input, 6);
    let exactly_6_set = potential_069
        .iter()
        .filter(|e| !e.is_superset(&exactly_1_set))
        .next()
        .unwrap();

    let exactly_c = exactly_1_set
        .difference(exactly_6_set)
        .next()
        .unwrap()
        .to_owned();

    let exactly_f = exactly_1_set
        .iter()
        .filter(|&&x| x != exactly_c)
        .next()
        .unwrap()
        .to_owned();

    let exactly_5_set = potential_235
        .iter()
        .filter(|x| !x.contains(&exactly_c))
        .next()
        .unwrap();

    let exactly_2_set = potential_235
        .iter()
        .filter(|&x| x != exactly_5_set && x != &exactly_3_set)
        .next()
        .unwrap();

    let exactly_e = exactly_2_set
        .difference(exactly_5_set)
        .copied()
        .filter(|&c| if c == exactly_c { false } else { true })
        .next()
        .unwrap();

    let exactly_b = exactly_8_set
        .difference(&exactly_3_set)
        .copied()
        .filter(|&c| if c == exactly_e { false } else { true })
        .next()
        .unwrap();

    let exactly_d = exactly_4
        .chars()
        .filter(|&c| match c {
            _ if c == exactly_b => false,
            _ if c == exactly_c => false,
            _ if c == exactly_f => false,
            _ => true,
        })
        .next()
        .unwrap();

    let exactly_g = exactly_3
        .chars()
        .filter(|&c| match c {
            _ if c == exactly_a => false,
            _ if c == exactly_c => false,
            _ if c == exactly_d => false,
            _ if c == exactly_f => false,
            _ => true,
        })
        .next()
        .unwrap();

    Mapping::new(
        exactly_a, exactly_b, exactly_c, exactly_d, exactly_e, exactly_f, exactly_g,
    )
}

#[aoc(day8, part2)]
fn part2(input: &[Entry]) -> u64 {
    input
        .iter()
        .map(|entry| {
            let mapping = derive_mapping(entry);
            entry
                .output
                .iter()
                .rev()
                .enumerate()
                .map(|(i, thing)| 10u64.pow(i as u32) * mapping.translate(thing))
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 26);
    }

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), 61229);
    }
}
