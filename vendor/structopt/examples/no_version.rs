<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[macro_use]
extern crate structopt;

use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "no_version",
    about = "",
    version = "",
    author = "",
    raw(global_settings = "&[AppSettings::DisableVersion]")
=======
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "no_version",
    no_version,
    global_settings = &[AppSettings::DisableVersion]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
)]
struct Opt {}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
