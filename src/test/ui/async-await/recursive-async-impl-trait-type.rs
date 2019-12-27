// edition:2018
// Test that impl trait does not allow creating recursive types that are
// otherwise forbidden when using `async` and `await`.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn recursive_async_function() -> () { //~ ERROR
    recursive_async_function().await;
}

fn main() {}
