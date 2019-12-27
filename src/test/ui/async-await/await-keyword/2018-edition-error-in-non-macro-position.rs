// edition:2018

#![allow(non_camel_case_types)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

mod outer_mod {
    pub mod await { //~ ERROR expected identifier, found reserved keyword `await`
        pub struct await; //~ ERROR expected identifier, found reserved keyword `await`
    }
}
use self::outer_mod::await::await; //~ ERROR expected identifier, found reserved keyword `await`
//~^ ERROR expected identifier, found reserved keyword `await`

struct Foo { await: () }
//~^ ERROR expected identifier, found reserved keyword `await`

impl Foo { fn await() {} }
//~^ ERROR expected identifier, found reserved keyword `await`

macro_rules! await {
//~^ ERROR expected identifier, found reserved keyword `await`
=======

mod outer_mod {
    pub mod await { //~ ERROR expected identifier, found keyword `await`
        pub struct await; //~ ERROR expected identifier, found keyword `await`
    }
}
use self::outer_mod::await::await; //~ ERROR expected identifier, found keyword `await`
//~^ ERROR expected identifier, found keyword `await`

struct Foo { await: () }
//~^ ERROR expected identifier, found keyword `await`

impl Foo { fn await() {} }
//~^ ERROR expected identifier, found keyword `await`

macro_rules! await {
//~^ ERROR expected identifier, found keyword `await`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    () => {}
}

fn main() {}
