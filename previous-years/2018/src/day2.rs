use aoc_runner_derive::aoc;
use std::collections::hash_map::HashMap;
use itertools::Itertools;

fn hash_line(line: &str) -> (i32, i32) {
    let mut counter = HashMap::new();
    for c in line.chars() {
       counter.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    let (hastwo, hasthree) = counter.values()
        .map(|&x| (x == 2, x == 3))
        .fold((false, false), |x, y| (x.0 || y.0, x.1 || y.1));

    (hastwo as i32, hasthree as i32)
}

#[aoc(day2, part1, Chars)]
fn part1(input: &str) -> i32 {
    let (twos, threes) = input.split_whitespace()
        .map(hash_line)
        .fold((0, 0), |x, y| (x.0 + y.0, x.1 + y.1));

    twos * threes
}

fn test_almost_similar(l: &str, r: &str) -> Option<String> {
    let common: String = l
        .chars()
        .zip(r.chars())
        .filter_map(|x| if x.0 == x.1 { Some(x.1) } else { None })
        .collect();
    if common.len() == l.len() - 1 {
        Some(common)
    } else {
        None
    }
}

#[aoc(day2, part2, Chars)]
fn part2(input: &str) -> String {
    input.split_whitespace()
        .combinations(2)
        .filter_map(|x| test_almost_similar(x[0], x[1]))
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let sample = "
            abcdef
            bababc
            abbcde
            abcccd
            aabcdd
            abcdee
            ababab
        ";
        assert_eq!(part1(sample), 12);
    }

    #[test]
    fn part2_sample() {
        let sample = "
            abcde
            fghij
            klmno
            pqrst
            fguij
            axcye
            wvxyz
        ";
        assert_eq!(part2(sample), "fgij");
    }
}
