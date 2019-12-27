<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[macro_use]
extern crate structopt;

use std::error::Error;
use structopt::StructOpt;

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "D", parse(try_from_str = "parse_key_val"))]
=======
use std::error::Error;
use structopt::StructOpt;

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(StructOpt, Debug)]
struct Opt {
    // number_of_values = 1 forces the user to repeat the -D option for each key-value pair:
    // my_program -D a=1 -D b=2
    // Without number_of_values = 1 you can do:
    // my_program -D a=1 b=2
    // but this makes adding an argument after the values impossible:
    // my_program -D a=1 -D b=2 my_input_file
    // becomes invalid.
    #[structopt(short = "D", parse(try_from_str = parse_key_val), number_of_values = 1)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    defines: Vec<(String, i32)>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
