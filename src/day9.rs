use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn generator(input: &str) -> usize {
    unimplemented!()
}

#[aoc(day9, part1)]
fn part1(input: &usize) -> usize {
    unimplemented!()
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

    #[test]
    fn test() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 13);
    }
}
