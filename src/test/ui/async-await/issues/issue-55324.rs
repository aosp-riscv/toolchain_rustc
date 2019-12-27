// build-pass (FIXME(62277): could be check-pass?)
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

use std::future::Future;

#[allow(unused)]
=======
use std::future::Future;

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn foo<F: Future<Output = i32>>(x: &i32, future: F) -> i32 {
    let y = future.await;
    *x + y
}

fn main() {}
