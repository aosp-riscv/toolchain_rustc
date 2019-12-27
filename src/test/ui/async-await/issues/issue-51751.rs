// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn inc(limit: i64) -> i64 {
    limit + 1
}

fn main() {
    let result = inc(10000);
    let finished = result.await;
    //~^ ERROR `await` is only allowed inside `async` functions and blocks
}
