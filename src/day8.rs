use aoc_runner_derive::{aoc, aoc_generator};
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
// The letter that 5 has that 2 does not is E.
// The letter that 3 is missing besides E is B.

// At this point we have identified A, B, C, E, and F.
// Take 4, the letter 4 has that we have not identified yet is D.
// Take 3.  The letter that 3 has that we have not identified is G.

struct Mapping {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
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

    unimplemented!()
}

#[aoc(day8, part2)]
fn part2(input: &[Entry]) -> i32 {
    unimplemented!()
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
        unimplemented!();
        // let data = generator(INPUT_TEXT);
        // assert_eq!(part2(&data), 168);
    }
}
