use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn generator(input: &str) -> usize {
    unimplemented!()
}

#[aoc(day5, part1)]
fn part1(input: &usize) -> String {
    unimplemented!()
}

#[aoc(day5, part2)]
fn part2(input: &usize) -> String {
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
}
