<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(raw(required = "true", min_values = "2"))]
=======
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(required = true, min_values = 2)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    foos: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
