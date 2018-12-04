use aoc_runner_derive::aoc;
use std::collections::hash_set::HashSet;

fn frequencies<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a + Clone {
    input
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
}

#[aoc(day1, part1, Chars)]
fn part1(input: &str) -> i32 {
    frequencies(input).sum()
}

#[aoc(day1, part2, Chars)]
fn part2(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut counter: i32 = 0;
    ::std::iter::once(0)
        .chain(frequencies(input).cycle())
        .filter_map(|x| {
            counter += x;
            if !seen.insert(counter) {
                Some(counter)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(part1("+1 +1 +1"), 3);
        assert_eq!(part1("+1 +1 -2"), 0);
        assert_eq!(part1("-1 -2 -3"), -6);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2("+1 -1"), 0);
        assert_eq!(part2("+3 +3 +4 -2 -4"), 10);
        assert_eq!(part2("-6 +3 +8 +5 -6"), 5);
        assert_eq!(part2("+7 +7 -2 -7 -4"), 14);
    }
}
