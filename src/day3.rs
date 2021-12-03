use aoc_runner_derive::{aoc, aoc_generator};

fn parse_binary(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            _ => unreachable!(),
        })
        .collect()
}

#[aoc_generator(day3)]
fn generator(input: &text) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(parse_binary)
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u8>]) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn tests_part1() {
        assert_eq!(part1(INPUT_TEXT), 198);
    }
}
