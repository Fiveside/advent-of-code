use utils;

pub fn day2(input_file: &str) {
    let input: Vec<Vec<i64>> = utils::filelines(input_file)
        .unwrap()
        .into_iter()
        .map(|x| parse_row(&x))
        .collect();

    println!("Day 2 part 1 solution: {}", part1(&input));
}

/*
--- Day 2: Corruption Checksum ---

As you walk through the door, a glowing humanoid shape yells in your direction. "You there! Your
state appears to be idle. Come help us repair the corruption in this spreadsheet - if we take
another millisecond, we'll have to display an hourglass cursor!"

The spreadsheet consists of rows of apparently-random numbers. To make sure the recovery process is
on the right track, they need you to calculate the spreadsheet's checksum. For each row, determine
the difference between the largest value and the smallest value; the checksum is the sum of all of
these differences.

For example, given the following spreadsheet:

5 1 9 5
7 5 3
2 4 6 8

* The first row's largest and smallest values are 9 and 1, and their difference is 8.
* The second row's largest and smallest values are 7 and 3, and their difference is 4.
* The third row's difference is 6.

In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
*/

fn checksum_row(row: &[i64]) -> i64 {
    let max = row.iter().cloned().fold(i64::min_value(), i64::max);
    let min = row.iter().cloned().fold(i64::max_value(), i64::min);
    max - min
}

fn part1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|x| checksum_row(x)).sum()
}

fn parse_row(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .map(|input| i64::from_str_radix(input, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_parse_row() {
        assert_eq!(parse_row("123\t456"), &[123, 456]);
    }

    #[test]
    fn test_day2_part1() {
        let sheet: Vec<Vec<i64>> = vec![
            vec![5, 1, 9, 5],
            vec![7, 5, 3],
            vec![2, 4, 6, 8],
        ];
        assert_eq!(part1(&sheet), 18);
    }
}