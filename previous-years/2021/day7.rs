use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp;
use std::collections::HashMap;

#[aoc_generator(day7)]
fn generator(input: &str) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = Default::default();
    for pos in input.split(',').map(|c| c.parse().unwrap()) {
        let entry = map.entry(pos).or_insert(0);
        *entry += 1;
    }
    return map;
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<i32, i32>) -> i32 {
    let mut lowest_gas = i32::MAX;
    for target in input.keys() {
        let mut gas = 0;
        for (key, val) in input.iter() {
            gas += (target - key).abs() * val;
        }
        lowest_gas = cmp::min(lowest_gas, gas);
    }
    return lowest_gas;
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<i32, i32>) -> i32 {
    let mut lowest_gas = i32::MAX;

    // The desired target may not be one of the ones in the input,
    // so create a range of all possible targets to make sure we hit it.
    let (smallest, largest) = input
        .keys()
        .cloned()
        .fold((i32::MAX, i32::MIN), |acc, item| {
            (cmp::min(acc.0, item), cmp::max(acc.1, item))
        });

    for target in smallest..=largest {
        let mut gas: i32 = 0;
        for (key, val) in input.iter() {
            let distance = (target - key).abs();
            let gas_required: i32 = (0..=distance).sum();
            gas += gas_required * val;
        }
        lowest_gas = cmp::min(lowest_gas, gas);
    }
    return lowest_gas;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 37);
    }

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), 168);
    }
}
