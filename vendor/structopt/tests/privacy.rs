// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[macro_use]
extern crate structopt;

mod options {
    #[derive(Debug, StructOpt)]
    pub struct Options {
        #[structopt(subcommand)]
        pub subcommand: ::subcommands::SubCommand,
    }
}

mod subcommands {
=======
use structopt::StructOpt;

mod options {
    use super::StructOpt;

    #[derive(Debug, StructOpt)]
    pub struct Options {
        #[structopt(subcommand)]
        pub subcommand: super::subcommands::SubCommand,
    }
}

mod subcommands {
    use super::StructOpt;

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    #[derive(Debug, StructOpt)]
    pub enum SubCommand {
        #[structopt(name = "foo", about = "foo")]
        Foo {
            #[structopt(help = "foo")]
            bars: Vec<String>,
        },
    }
}
