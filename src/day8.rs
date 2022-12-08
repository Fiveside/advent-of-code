use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn generator(input: &str) -> usize {
    unimplemented!()
}

#[aoc(day8, part1)]
fn part1(input: &usize) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 21)
    }
}
