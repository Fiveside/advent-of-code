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

fn find_indexed_rating(numbers: &[&Vec<u8>], index: usize) -> u32 {
    let mut rating = 0;
    for number in numbers {
        rating += number[index] as u32;
    }
    return rating;
}

fn filter_indexed_rating<'a>(
    numbers: &[&'a Vec<u8>],
    desired: u8,
    index: usize,
) -> Vec<&'a Vec<u8>> {
    numbers
        .into_iter()
        .map(|x| *x) // I don't understand why this is neccesary. Why do we end up with &&Vec?
        .filter(|x| x[index] == desired)
        .collect()
}

fn filter_rating<'a>(mut numbers: Vec<&'a Vec<u8>>, lsr: LifeSupportRating) -> &'a Vec<u8> {
    let mut idx = 0;
    loop {
        if numbers.len() == 1 {
            return numbers.into_iter().next().unwrap();
        }
        let rating = find_indexed_rating(&numbers, idx);

        numbers =
            filter_indexed_rating(&numbers, lsr.get_desired_value(rating, numbers.len()), idx);
        idx += 1;
    }
}

enum LifeSupportRating {
    Oxygen,
    CO2,
}

impl LifeSupportRating {
    fn get_desired_value(&self, meta: u32, count: usize) -> u8 {
        match self {
            &LifeSupportRating::Oxygen => {
                if meta as f32 >= count as f32 / 2f32 {
                    1
                } else {
                    0
                }
            }
            &LifeSupportRating::CO2 => {
                if (meta as f32) < count as f32 / 2f32 {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u8>]) -> i64 {
    // let numbers: HashSet<Vec<u8>> = HashSet::from(input.iter());
    let mut oxy_numbers = Vec::with_capacity(input.len());
    for line in input.iter() {
        oxy_numbers.push(line);
    }
    let co2_numbers = oxy_numbers.clone();

    let oxy_val = filter_rating(oxy_numbers, LifeSupportRating::Oxygen);
    let oxy_str: String = oxy_val.iter().map(|x| x.to_string()).collect();

    let co2_val = filter_rating(co2_numbers, LifeSupportRating::CO2);
    let co2_str: String = co2_val.iter().map(|x| x.to_string()).collect();

    let oxy = i64::from_str_radix(&oxy_str, 2).unwrap();
    let co2 = i64::from_str_radix(&co2_str, 2).unwrap();

    return oxy * co2;
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

    #[test]
    fn tests_part2() {
        let parsed = generator(INPUT_TEXT);
        assert_eq!(part2(&parsed), 230)
    }
}
