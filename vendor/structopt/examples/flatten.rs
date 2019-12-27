<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[macro_use]
extern crate structopt;

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cmdline {
    #[structopt(short = "v", help = "switch on verbosity")]
    verbose: bool,
    #[structopt(flatten)]
    daemon_opts: DaemonOpts,
}

#[derive(StructOpt, Debug)]
struct DaemonOpts {
    #[structopt(short = "u", help = "daemon user")]
    user: String,
    #[structopt(short = "g", help = "daemon group")]
    group: String,
}

fn main() {
    let opt = Cmdline::from_args();
    println!("{:?}", opt);
}
