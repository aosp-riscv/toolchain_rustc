// edition:2018
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

// Regression test for issue #62382
=======

// Regression test for issue #62382.
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

use std::future::Future;

fn get_future() -> impl Future<Output = ()> {
    panic!()
}

async fn foo() {
    let a; //~ ERROR type inside `async` object must be known in this context
    get_future().await;
}

fn main() {}
