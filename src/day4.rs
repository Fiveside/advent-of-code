use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct BingoBoard {
    raw_board: Vec<Vec<u32>>,
    solutions: Vec<HashSet<u32>>,
}

impl BingoBoard {
    // Example board text:
    //     22 13 17 11  0
    //     8  2 23  4 24
    //    21  9 14 16  7
    //     6 10  3 18  5
    //     1 12 20 15 19
    fn from_text(text: &str) -> Self {
        let board: Vec<Vec<u32>> = text
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|num| u32::from_str_radix(num, 10).unwrap())
                    .collect()
            })
            .collect();

        let solutions = find_solutions(&board);
        Self {
            raw_board: board,
            solutions,
        }
    }

    fn partial_score(&self, drawn_numbers: &HashSet<u32>) -> u32 {
        self.raw_board
            .iter()
            .flatten()
            .filter(|&x| !drawn_numbers.contains(x))
            .sum()
    }

    fn has_won(&self, picked: &HashSet<u32>) -> bool {
        self.solutions.iter().any(|x| picked.is_superset(x))
    }
}

fn find_solutions(board: &[Vec<u32>]) -> Vec<HashSet<u32>> {
    // Too lazy to generalize at the moment.  Magic numbers in this function
    // relate to these assertions.
    assert!(board.len() == 5);
    for line in board {
        assert!(line.len() == 5);
    }

    let mut solutions = Vec::with_capacity(12);

    // First find all horizontal solutions
    for horizontal_line in board {
        solutions.push(HashSet::from_iter(horizontal_line.iter().cloned()));
    }

    // Find all vertical solutions
    for idx in 0..board[0].len() {
        let mut solution = HashSet::with_capacity(board.len());
        for horizontal_line in board {
            solution.insert(horizontal_line[idx]);
        }
        solutions.push(solution);
    }

    // Apparently diagonal solutions are not considered in this exercise.

    // // Top left horizontal solution
    // let mut topleft = HashSet::with_capacity(5);
    // topleft.insert(board[0][0]);
    // topleft.insert(board[1][1]);
    // topleft.insert(board[2][2]);
    // topleft.insert(board[3][3]);
    // topleft.insert(board[4][4]);
    // solutions.push(topleft);

    // // Top right horizontal solution
    // let mut topright = HashSet::with_capacity(5);
    // topright.insert(board[4][0]);
    // topright.insert(board[3][1]);
    // topright.insert(board[2][2]);
    // topright.insert(board[1][3]);
    // topright.insert(board[0][4]);
    // solutions.push(topright);

    return solutions;
}

#[derive(Debug)]
struct BingoGame {
    drawings: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn new(drawings: Vec<u32>, boards: Vec<BingoBoard>) -> Self {
        Self { drawings, boards }
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> BingoGame {
    // Split objects in the input based on fresh newline
    let re = Regex::new(r"\n\s*\n").unwrap();
    let mut it = re.split(input).filter(|x| x.trim().len() > 0);
    let drawings = it
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let boards = it.map(BingoBoard::from_text).collect();

    BingoGame::new(drawings, boards)
}

#[aoc(day4, part1)]
fn part1(game: &BingoGame) -> u32 {
    let mut picked = HashSet::new();
    let mut number_picker = game.drawings.iter().cloned();
    loop {
        let last_number = number_picker.next().unwrap();
        picked.insert(last_number);
        let wins: Vec<_> = game.boards.iter().filter(|x| x.has_won(&picked)).collect();
        if wins.len() == 0 {
            // continue looking for a winner
            continue;
        }

        // At least one winner found, get the highest scoring board
        let pscore = wins
            .into_iter()
            .map(|board| board.partial_score(&picked))
            .max()
            .unwrap();
        return pscore * last_number;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        let processed = generator(INPUT_TEXT);
        assert_eq!(part1(&processed), 4512);
    }
}
