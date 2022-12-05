use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Day5 {
    state: Vec<Vec<char>>,
    operations: Vec<Operation>,
}

#[derive(Debug)]
struct Operation {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_state(input: &str) -> Vec<Vec<char>> {
    // my god the string manip here sucks. I must be doing something wrong holy moly.
    //
    // The state definition is a big block that is wicked annoying to parse and
    // needs to be roated to make it easier.  Separate each string into blocks
    // of 4 characters and rotate them.  The last line is one guaranteed to have
    // the maximum number of blocks without padding with spaces.

    let num_lines = input.lines().count();
    let last_line = input.lines().last().unwrap();
    let num_cols = last_line
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let others = input
        .lines()
        .take(num_lines - 1)
        .map(|line| {
            let mut buf = Vec::with_capacity(line.len() / 4);
            for chunk in &line.chars().chunks(4) {
                let s = chunk.collect::<String>();
                buf.push(s);
            }
            buf
        })
        .collect::<Vec<Vec<String>>>();

    // Turn all others into a mutable vector of iterators, and then for each
    // rotated line, pick one chunk from top to bottom.
    let mut its = others
        .into_iter()
        .map(|line| line.into_iter())
        .collect::<Vec<_>>();
    its.reverse();

    let mut stack_string = Vec::new();
    for _ in 0..num_cols {
        let mut string_buf = Vec::with_capacity(its.len());
        for it in its.iter_mut() {
            string_buf.push(it.next().unwrap_or("    ".to_string()))
        }
        stack_string.push(string_buf.into_iter().join(""))
    }

    stack_string
        .into_iter()
        .map(|s| {
            s.chars()
                .filter(|c| c.is_alphabetic())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>()
}

fn parse_operations(input: &str) -> Vec<Operation> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| Operation {
            count: cap.get(1).unwrap().as_str().parse().unwrap(),
            from: cap.get(2).unwrap().as_str().parse().unwrap(),
            to: cap.get(3).unwrap().as_str().parse().unwrap(),
        })
        .collect()
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Day5 {
    let (state_str, operations_str) = input.split_once("\n\n").unwrap();
    let state = parse_state(state_str);
    let operations: Vec<Operation> = parse_operations(operations_str);
    Day5 { state, operations }
}

#[aoc(day5, part1)]
fn part1(input: &Day5) -> String {
    let mut state = input.state.clone();
    for operation in input.operations.iter() {
        for _ in 0..(operation.count) {
            let thing = state[operation.from - 1].pop().unwrap();
            state[operation.to - 1].push(thing);
        }
    }

    state.into_iter().map(|mut v| v.pop().unwrap()).collect()
}

#[aoc(day5, part2)]
fn part2(input: &Day5) -> String {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), "CMZ");
    }

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), "MCD");
    }
}
