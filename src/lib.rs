mod aoc;
mod day1;
mod day2;
mod day3;
mod input;
mod util;

use aoc::{AocYear, Day};
use util::INPUT_LOCATION;

macro_rules! day_runner {
    ($impl:path) => {
        Some({
            fn day_implementation(input: &str) -> String {
                format!("{}", $impl(input))
            }
            day_implementation
        })
    };
    ($impl:path, $gen:path) => {
        Some({
            fn day_implementation(input: &str) -> String {
                let data = $gen(input);
                let res = $impl(&data);
                format!("{}", res)
            }
            day_implementation
        })
    };
}

pub fn year_2022() -> AocYear {
    AocYear {
        year: 2022,
        days: vec![
            Day {
                input: include_str!(INPUT_LOCATION + "/2022/day1.txt"),
                part1: day_runner!(day1::part1, day1::generator),
                part2: day_runner!(day1::part2, day1::generator),
            },
            Day {
                input: include_str!("../input/2022/day2.txt"),
                part1: day_runner!(day2::part1, day2::generator),
                part2: day_runner!(day2::part2, day2::generator),
            },
        ],
    }
}
