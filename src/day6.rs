use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashSet;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    for (idx, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return idx + 4;
        }
    }
    unreachable!();
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    for (idx, window) in chars.windows(14).enumerate() {
        let mut set = HashSet::with_capacity(14);
        for c in window {
            set.insert(*c);
        }
        if set.len() == 14 {
            return idx + 14;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_SAMPLES_PART1: [(&'static str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    const INPUT_SAMPLES_PART2: [(&'static str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    #[test]
    fn test_part1() {
        for (input, expect) in INPUT_SAMPLES_PART1 {
            assert_eq!(part1(input), expect);
        }
    }

    #[test]
    fn test_part2() {
        for (input, expect) in INPUT_SAMPLES_PART2 {
            assert_eq!(part2(input), expect);
        }
    }
}
