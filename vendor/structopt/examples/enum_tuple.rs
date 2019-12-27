<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[macro_use]
extern crate structopt;

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Foo {
    pub bar: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "foo")]
    Foo(Foo),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "classify")]
pub struct ApplicationArguments {
    #[structopt(subcommand)]
    pub command: Command,
}

fn main() {
    let opt = ApplicationArguments::from_args();
    println!("{:?}", opt);
}
