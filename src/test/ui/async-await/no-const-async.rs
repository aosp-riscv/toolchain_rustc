// compile-fail
// edition:2018
// compile-flags: --crate-type lib

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

pub const async fn x() {}
//~^ ERROR expected identifier, found reserved keyword `async`
=======
pub const async fn x() {}
//~^ ERROR expected identifier, found keyword `async`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
//~^^ expected `:`, found keyword `fn`
