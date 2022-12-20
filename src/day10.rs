use aoc_runner_derive::{aoc, aoc_generator};

struct VirtualMachine<'a> {
    x: i32,
    cycle: u32,

    interrupts: &'a [u32],
    instructions: Vec<Instruction>,

    // Number of cycles before applying the next instruction
    next_cycles: u32,

    // Next instrution to apply.
    next: Instruction,
}

impl<'a> VirtualMachine<'a> {
    fn new(interrupts: &'a [u32], mut instructions: Vec<Instruction>) -> Self {
        instructions.reverse();
        let next = instructions.pop().unwrap();
        Self {
            x: 1,
            cycle: 0,
            interrupts,
            instructions,

            next_cycles: next.cycles(),
            next,
        }
    }

    fn tick_part1(&mut self) -> Option<Option<i32>> {
        let instruction = match self.instructions.pop() {
            Some(x) => x,
            None => return None,
        };

        self.cycle += instruction.cycles();
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
        if let Instruction::Addx(x) = instruction {
            self.x += x;
        }

        Some(res)
    }

    fn generate_display(&mut self) -> [[bool; 40]; 6] {
        let mut display = [
            [false; 40],
            [false; 40],
            [false; 40],
            [false; 40],
            [false; 40],
            [false; 40],
        ];
        let mut cursor: usize = 0;

        'instruction: loop {
            let cursor_in_range = (self.x - 1..=self.x + 1)
                .into_iter()
                .any(|x| x == (cursor % 40).try_into().unwrap());
            if cursor_in_range {
                display[cursor / 40][cursor % 40] = true;
            }
            cursor += 1;
            self.cycle += 1;

            if self.next_cycles == 0 {
                match self.next {
                    Instruction::Addx(next_x) => self.x += next_x,
                    Instruction::Noop => {
                        // nothing to do here.
                    }
                }
                if cursor == 40 * 6 {
                    break 'instruction;
                }
                match self.instructions.pop() {
                    Some(instruction) => {
                        self.next_cycles = instruction.cycles();
                        self.next = instruction;
                    }
                    // No more instructions, bail.
                    None => break 'instruction,
                }
            } else {
                self.next_cycles -= 1;
            }
        }

        return display;
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Self::Addx(_) => 1,
            Self::Noop => 0,
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

// part 1 is super broken
#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let interrupts = vec![20, 60, 100, 140, 180, 220];
    let mut signals = Vec::with_capacity(interrupts.len());
    let mut machine = VirtualMachine::new(&interrupts, input.to_vec());

    while let Some(x) = machine.tick_part1() {
        if let Some(y) = x {
            signals.push(y);
        }
    }

    return signals
        .into_iter()
        .zip(interrupts.into_iter())
        .map(|(x, y)| x * (y as i32))
        .sum();
}

#[aoc(day10, part2)]
fn part2(input: &[Instruction]) -> String {
    let mut machine = VirtualMachine::new(&[], input.to_vec());

    let display = machine.generate_display();
    let result = display
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|x| if x { '🦾' } else { '🍙' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", result);
    return result;
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

    #[test]
    fn test_part2() {
        let data = generator(INPUT_TEXT);
        assert_eq!(part2(&data), "🦾🦾🍙🍙🦾🦾🍙🍙🦾🦾🍙🍙🦾🦾🍙🍙🦾🦾🍙🍙🦾🦾🍙🍙🦾🦾🍙🍙🦾🦾🍙🍙🦾🦾🍙🍙🦾🦾🍙🍙\n🦾🦾🦾🍙🍙🍙🦾🦾🦾🍙🍙🍙🦾🦾🦾🍙🍙🍙🦾🦾🦾🍙🍙🍙🦾🦾🦾🍙🍙🍙🦾🦾🦾🍙🍙🍙🦾🦾🦾🍙\n🦾🦾🦾🦾🍙🍙🍙🍙🦾🦾🦾🦾🍙🍙🍙🍙🦾🦾🦾🦾🍙🍙🍙🍙🦾🦾🦾🦾🍙🍙🍙🍙🦾🦾🦾🦾🍙🍙🍙🍙\n🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙\n🦾🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙🍙🦾🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙🍙🦾🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙🍙🦾🦾🦾🦾\n🦾🦾🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙🍙🍙🦾🦾🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙🍙🍙🦾🦾🦾🦾🦾🦾🦾🍙🍙🍙🍙🍙")
    }
}
