use aoc_runner_derive::{aoc, aoc_generator};

fn parse_binary(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            _ => unreachable!(),
        })
        .collect()
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(parse_binary)
        .collect()
}

fn gamma_from_meta(value: u64, thresh: usize) -> u8 {
    if value as usize > thresh {
        1
    } else {
        0
    }
}

fn epsilon_from_meta(value: u64, thresh: usize) -> u8 {
    if (value as usize) < thresh {
        1
    } else {
        0
    }
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u8>]) -> i64 {
    let numlines = input.len();
    let buflen = input[0].len();
    let mut buf: Vec<u64> = Vec::with_capacity(buflen);
    for _ in 0..buflen {
        buf.push(0);
    }

    for line in input {
        for (idx, val) in line.iter().enumerate() {
            buf[idx] += *val as u64;
        }
    }

    // buf now contains the meta analysis
    let gamma: String = buf
        .iter()
        .map(|x| gamma_from_meta(*x, numlines / 2).to_string())
        .collect();
    let epsilon: String = buf
        .iter()
        .map(|x| epsilon_from_meta(*x, numlines / 2).to_string())
        .collect();

    let g_num = i64::from_str_radix(&gamma, 2).unwrap();
    let e_num = i64::from_str_radix(&epsilon, 2).unwrap();

    return g_num * e_num;
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEXT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn tests_part1() {
        let parsed = generator(INPUT_TEXT);
        assert_eq!(part1(&parsed), 198);
    }
}
