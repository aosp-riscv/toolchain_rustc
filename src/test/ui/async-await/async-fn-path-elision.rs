// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]
#![allow(dead_code)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct HasLifetime<'a>(&'a bool);

async fn error(lt: HasLifetime) { //~ ERROR implicit elided lifetime not allowed here
    if *lt.0 {}
}

fn no_error(lt: HasLifetime) {
    if *lt.0 {}
}

fn main() {}
