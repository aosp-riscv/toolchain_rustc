// edition:2018
#![allow(non_camel_case_types)]

mod outer_mod {
    pub mod await { //~ ERROR expected identifier
        pub struct await; //~ ERROR expected identifier
    }
}
use self::outer_mod::await::await; //~ ERROR expected identifier
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR expected identifier, found reserved keyword `await`

macro_rules! await { () => {}; } //~ ERROR expected identifier, found reserved keyword `await`
=======
    //~^ ERROR expected identifier, found keyword `await`

macro_rules! await { () => {}; } //~ ERROR expected identifier, found keyword `await`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

fn main() {
    await!(); //~ ERROR expected expression, found `)`
}
