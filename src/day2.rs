use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Action {
    fn parse_action_line(input: &str) -> Self {
        // expects stuff of the form "Foo N" where N is a positive integer.
        let (word, mag_str) = input.split_whitespace().collect_tuple().unwrap();
        let magnitude = mag_str.parse().unwrap();

        match word {
            "forward" => Action::Forward(magnitude),
            "up" => Action::Up(magnitude),
            "down" => Action::Down(magnitude),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct Position {
    forward: u32,
    depth: i64,
}

impl Position {
    fn new() -> Self {
        Position {
            forward: 0,
            depth: 0,
        }
    }

    fn move_position(&mut self, act: &Action) {
        match act {
            &Action::Forward(x) => self.forward += x,
            &Action::Down(x) => self.depth += x as i64,
            &Action::Up(x) => self.depth -= x as i64,
        }
    }

    fn answer_part1(self) -> i64 {
        (self.forward as i64) * self.depth
    }
}

#[aoc_generator(day2)]
fn input_generation(input: &str) -> Vec<Action> {
    input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| Action::parse_action_line(s))
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Action]) -> i64 {
    let mut pos = Position::new();

    for action in input {
        pos.move_position(action);
    }

    pos.answer_part1()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    const INPUT_ACTIONS: [Action; 6] = [
        Action::Forward(5),
        Action::Down(5),
        Action::Forward(8),
        Action::Up(3),
        Action::Down(8),
        Action::Forward(2),
    ];

    #[test]
    fn test_generator() {
        assert_eq!(&input_generation(INPUT_TEXT), &INPUT_ACTIONS);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT_ACTIONS), 150);
    }
}
