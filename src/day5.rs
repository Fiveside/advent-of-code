use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Day5 {
    state: StackState,
    operations: Vec<Operation>,
}

#[derive(Debug)]
struct StackState {
    stacks: Vec<char>,
}

#[derive(Debug)]
struct Operation {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_state(input: &str) -> StackState {
    let last_line = input.lines().last().unwrap();
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
    let state: StackState = parse_state(state_str);
    let operations: Vec<Operation> = parse_operations(operations_str);
    Day5 { state, operations }
}

#[aoc(day5, part1)]
fn part1(input: &Day5) -> String {
    unimplemented!()
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
        let data = dbg!(generator(INPUT_TEXT));
        assert_eq!(part1(&data), "CMZ");
    }
}
