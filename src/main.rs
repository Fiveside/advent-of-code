extern crate itertools;
extern crate structopt;

#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

mod utils;
mod day1;
mod day2;

#[derive(StructOpt, Debug)]
#[structopt]
enum Opt {
    #[structopt(name = "1", help = "Run day 1 solution")]
    Day1 {
        #[structopt]
        captcha: String,
    },
    #[structopt(name = "2", help = "Run day 2 solution")]
    Day2 {
        #[structopt]
        input_file: String,
    }
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Day1{captcha: x} => day1::day1(&x),
        Opt::Day2{input_file: x} => day2::day2(&x),
    };
}
