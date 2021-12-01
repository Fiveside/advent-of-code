use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

#[aoc_generator(day1)]
fn input_generation(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<u32>) -> u32 {
    input
        .iter()
        .tuple_windows()
        .map(|(l, r)| if r > l { 1 } else { 0 })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<u32>) -> u32 {
    input
        .iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows()
        .map(|(l, r)| if r > l { 1 } else { 0 })
        .sum()
}
