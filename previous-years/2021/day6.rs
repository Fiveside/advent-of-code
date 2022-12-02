use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<u64> {
    let mut buf = vec![0u64; 9];
    for thing in input.split(',') {
        let count: usize = thing.parse().unwrap();
        buf[count] += 1;
    }

    return buf;
}

fn simulate(input: &[u64], days: u32) -> u64 {
    // let mut deque = VecDeque::from(input);
    let mut deque: VecDeque<u64> = input.iter().cloned().collect();
    for _ in 0..days {
        let latest = deque.pop_front().unwrap();

        // Add the latest generation in as new fish
        deque.push_back(latest);

        // Reset the latest generation's gestation period
        let pos = deque.get_mut(6).unwrap();
        *pos += latest;
    }

    return deque.into_iter().sum();
}

#[aoc(day6, part1)]
fn part1(input: &[u64]) -> u64 {
    simulate(input, 80)
}

#[aoc(day6, part2)]
fn part2(input: &[u64]) -> u64 {
    simulate(input, 256)
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

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), 26984457539);
    }
}
