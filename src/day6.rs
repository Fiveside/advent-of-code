use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn generator(input: &str) -> u32 {
    unimplemented!();
}

#[aoc(day6, part1)]
fn part1(input: u32) -> u32 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 5934);
    }
}
