use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<(char, char)> {
    input.lines().map(|line| {
        let mut c = line.chars();
        let left = c.next().unwrap();
        let right = c.skip(1).next().unwrap();
        (left, right)
    }).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(char, char)]) -> u32 {
    input.iter().fold(0, |acc, &(opponent, me)|
        acc + match (opponent, me) {
            // my_score + game_result
            ('A', 'X') => 1 + 3,
            ('A', 'Y') => 2 + 6,
            ('A', 'Z') => 3 + 0,
            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,
            ('C', 'X') => 1 + 6,
            ('C', 'Y') => 2 + 0,
            ('C', 'Z') => 3 + 3,
            _ => unreachable!()
        }
    )
}

#[aoc(day2, part2)]
fn part2(input: &[(char, char)]) -> u32 {
    input.iter().fold(0, |acc, &(opponent, me)|
        acc + match (opponent, me) {
            // my_score + game_result
            ('A', 'X') => 3 + 0,
            ('A', 'Y') => 1 + 3,
            ('A', 'Z') => 2 + 6,
            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,
            ('C', 'X') => 2 + 0,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 1 + 6,
            _ => unreachable!()
        }
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let parsed = generator(INPUT_TEXT);
        assert_eq!(part1(&parsed), 15);
    }

    #[test]
    fn test_part2() {
        let parsed = generator(INPUT_TEXT);
        assert_eq!(part2(&parsed), 12);
    }
}
