// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await, async_closure)]
=======
#![feature(async_closure)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

async fn print_dur() {}

fn main() {
    (async || 2333)().await;
    //~^ ERROR `await` is only allowed inside `async` functions and blocks
}
