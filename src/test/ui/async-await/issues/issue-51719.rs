// edition:2018
//
// Tests that the .await syntax can't be used to make a generator

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn foo() {}

fn make_generator() {
    let _gen = || foo().await;
    //~^ ERROR `await` is only allowed inside `async` functions and blocks
}

fn main() {}
