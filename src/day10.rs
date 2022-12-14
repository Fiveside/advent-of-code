use aoc_runner_derive::{aoc, aoc_generator};

struct VirtualMachine<'a> {
    x: i32,
    cycle: u32,

    interrupts: &'a [u32],
}

impl<'a> VirtualMachine<'a> {
    fn new(interrupts: &'a [u32]) -> Self {
        Self {
            x: 1,
            cycle: 0,
            interrupts,
        }
    }

    fn apply(&mut self, that: &Instruction) -> Option<i32> {
        self.cycle += that.cycles();
        let res = if let Some(x) = self.interrupts.first() {
            if self.cycle >= *x {
                self.interrupts = &self.interrupts[1..];
                Some(self.x)
            } else {
                None
            }
        } else {
            None
        };

        // This is the only instruction that changes
        // the actual machine state.
        if let Instruction::Addx(x) = that {
            self.x += *x;
        }

        res
    }
}

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }
}

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            if line == "noop" {
                return Instruction::Noop;
            }
            let val = line.rsplit_once(" ").unwrap().1.parse().unwrap();
            return Instruction::Addx(val);
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let interrupts = vec![20, 60, 100, 140, 180, 220];
    let mut signals = Vec::with_capacity(interrupts.len());
    let mut machine = VirtualMachine::new(&interrupts);

    for instruction in input {
        if let Some(x) = machine.apply(instruction) {
            signals.push(x);
        }
    }

    signals
        .into_iter()
        .zip(interrupts.into_iter())
        .map(|(x, y)| x * (y as i32))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part1() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part1(&data), 13140);
    }
}
