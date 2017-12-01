extern crate itertools;
extern crate structopt;

#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

mod day1;

#[derive(StructOpt, Debug)]
#[structopt]
enum Opt {
    #[structopt(name = "1", help = "Run day 1 solution")]
    Day1 {
        #[structopt]
        captcha: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Day1{captcha: x} => day1::day1((&x))
    };
}
