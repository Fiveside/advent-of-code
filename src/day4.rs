use aoc_runner_derive::{aoc, aoc_generator};

struct Assignment(u32, u32);

impl Assignment {
    fn from_str(input: &str) -> Self {
        let (l, r) = input.split_once('-').unwrap();
        Self(l.parse().unwrap(), r.parse().unwrap())
    }

    fn fully_contains(&self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (Assignment::from_str(l), Assignment::from_str(r))
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[(Assignment, Assignment)]) -> u32 {
    input
        .iter()
        .filter(|&(l, r)| l.fully_contains(r) || r.fully_contains(l))
        .count()
        .try_into()
        .unwrap()
}

#[aoc(day4, part2)]
fn part2(input: &[(Assignment, Assignment)]) -> u32 {
    input
        .iter()
        .filter(|&(l, r)| l.overlaps(r))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 2);
    }

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), 4);
    }
}
