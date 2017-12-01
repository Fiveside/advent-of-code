extern crate structopt;

#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt]
enum Opt {
    #[structopt(name = "1")]
    Day1 {
        #[structopt]
        captcha: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
