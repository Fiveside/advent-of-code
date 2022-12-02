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
                .trim()
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
    elves.reverse();
    return elves.iter().take(3).map(|x| x.caloric_total()).sum();
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_part1() {
        let input = input_generation(INPUT_TEXT);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn test_part2() {
        let input = input_generation(INPUT_TEXT);
        assert_eq!(part2(&input), 45000);
    }

}
