use std::hash::Hash;
use std::{collections::HashSet, fmt::Display};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

struct Instruction {
    direction: Direction,
    magnitude: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance(&self, other: &Position) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    fn move_direction(self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r#"(\w)\s+(\d+)"#).unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let direction =
                Direction::from_char(caps.get(1).unwrap().as_str().chars().next().unwrap());
            let magnitude = caps.get(2).unwrap().as_str().parse().unwrap();
            Instruction {
                direction,
                magnitude,
            }
        })
        .collect()
}

fn new_tail(head: &Position, tail: &Position) -> Position {
    let distance = head.distance(tail);

    let shouldmove = (head.x - tail.x).abs().max((head.y - tail.y).abs());
    if shouldmove <= 1 {
        // no adjustment needed.
        return *tail;
    }

    let xmove = tail.x - head.x;
    let ymove = tail.y - head.y;
    if xmove.abs() == ymove.abs() {
        // Moved both x and y, this is a diagnoal tug.
        return Position {
            x: head.x + xmove.signum(),
            y: head.y + ymove.signum(),
        };
    }

    if xmove.abs() >= 2 {
        return Position {
            x: head.x + xmove.signum(),
            y: head.y,
        };
    }

    if ymove.abs() >= 2 {
        return Position {
            x: head.x,
            y: head.y + ymove.signum(),
        };
    }
    unreachable!()
}

#[aoc(day9, part1)]
fn part1(input: &[Instruction]) -> usize {
    let mut positions = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = head.clone();
    positions.insert(tail);

    for instruction in input {
        for _ in 0..instruction.magnitude {
            head = head.move_direction(&instruction.direction);
            tail = new_tail(&head, &tail);
            positions.insert(tail);
        }
    }

    positions.len()
}

#[aoc(day9, part2)]
fn part2(input: &[Instruction]) -> usize {
    let mut positions = HashSet::new();
    let mut knots = vec![Position::default(); 10];
    for instruction in input {
        for _ in 0..instruction.magnitude {
            // move head
            knots[0] = knots[0].move_direction(&instruction.direction);

            // move knots
            for idx in 1..knots.len() {
                knots[idx] = new_tail(&knots[idx - 1], &knots[idx]);
            }

            positions.insert(*knots.last().unwrap());
        }
    }

    positions.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const MORE_INPUT_TEXT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 13);
    }

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), 1);
    }

    #[test]
    fn test_part2_more() {
        let data = generator(MORE_INPUT_TEXT);
        assert_eq!(part2(&data), 36);
    }
}
