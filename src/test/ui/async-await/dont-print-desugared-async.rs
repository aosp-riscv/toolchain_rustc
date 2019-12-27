// Test that we don't show variables with from async fn desugaring

// edition:2018
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

async fn async_fn(&ref mut s: &[i32]) {}
//~^ ERROR cannot borrow data in a `&` reference as mutable [E0596]

fn main() {}
