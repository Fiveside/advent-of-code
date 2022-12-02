use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn caloric_total(&self) -> u32 {
        self.calories.iter().sum()
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.caloric_total().partial_cmp(&other.caloric_total())
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.caloric_total() {
            x if x < other.caloric_total() => std::cmp::Ordering::Less,
            x if x == other.caloric_total() => std::cmp::Ordering::Equal,
            _ => std::cmp::Ordering::Greater,
        }
    }
}

#[aoc_generator(day1)]
fn input_generation(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|raw_lines| Elf {
            calories: raw_lines
                .split('\n')
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Elf]) -> u32 {
    let elf = input.iter().max().unwrap();
    return elf.caloric_total();
}

#[aoc(day1, part2)]
fn part2(input: &[Elf]) -> u32 {
    let mut elves = input.to_vec();
    elves.sort();
    return elves.iter().take(3).map(|x| x.caloric_total()).sum();
}
