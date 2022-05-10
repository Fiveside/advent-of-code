use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    fn is_cardinal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn covered_positions(&self) -> Vec<Position> {
        if self.start.x == self.end.x {
            // horizontal case
            let start = cmp::min(self.start.y, self.end.y);
            let end = cmp::max(self.start.y, self.end.y);
            return (start..=end)
                .map(|y| Position::new(self.start.x, y))
                .collect();
        } else if self.start.y == self.end.y {
            // vertical case
            let start = cmp::min(self.start.x, self.end.x);
            let end = cmp::max(self.start.x, self.end.x);
            return (start..=end)
                .map(|x| Position::new(x, self.start.y))
                .collect();
        } else {
            // diagonal case, assume perfect 45 degree lines because of the problem space
            let xdirection = get_sign(self.end.x - self.start.x);
            let ydirection = get_sign(self.end.y - self.start.y);
            let magnitude = (self.end.x - self.start.x).abs();
            return (0..=magnitude)
                .map(|mag| {
                    Position::new(
                        self.start.x + (mag * xdirection),
                        self.start.y + (mag * ydirection),
                    )
                })
                .collect();
        }
    }
}

fn get_sign(x: i32) -> i32 {
    if x < 0 {
        -1
    } else {
        1
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<Line> {
    let matcher =
        Regex::new(r"\s*(?P<x1>\d+),(?P<y1>\d+)\s*->\s*(?P<x2>\d+),(?P<y2>\d+)\s*").unwrap();

    input
        .split("\n")
        .map(|line| {
            let caps = matcher.captures(line).unwrap();
            Line::new(
                Position::new(caps["x1"].parse().unwrap(), caps["y1"].parse().unwrap()),
                Position::new(caps["x2"].parse().unwrap(), caps["y2"].parse().unwrap()),
            )
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[Line]) -> u32 {
    let mut points = HashSet::new();
    let mut double_covered_points = HashSet::new();
    for line in input.iter().filter(|l| l.is_cardinal()) {
        for pos in line.covered_positions() {
            if !points.insert(pos) {
                // position already existed in the set
                double_covered_points.insert(pos);
            }
        }
    }

    return double_covered_points.len() as u32;
}

#[aoc(day5, part2)]
fn part2(input: &[Line]) -> u32 {
    let mut points = HashSet::new();
    let mut double_covered_points = HashSet::new();
    for line in input {
        for pos in line.covered_positions() {
            if !points.insert(pos) {
                double_covered_points.insert(pos);
            }
        }
    }

    return double_covered_points.len() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 5);
    }

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), 12);
    }
}
