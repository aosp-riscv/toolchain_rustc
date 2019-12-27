// build-pass (FIXME(62277): could be check-pass?)
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

use std::future::Future;

#[allow(unused)]
=======
use std::future::Future;

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn enter<'a, F, R>(mut callback: F)
where
    F: FnMut(&'a mut i32) -> R,
    R: Future<Output = ()> + 'a,
{
    unimplemented!()
}

fn main() {}
