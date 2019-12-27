// Output = String caused an ICE whereas Output = &'static str compiled successfully.
// Broken MIR: generator contains type std::string::String in MIR,
// but typeck only knows about {<S as T>::Future, ()}
// check-pass
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
use std::future::Future;

pub trait T {
    type Future: Future<Output = String>;
    fn bar() -> Self::Future;
}
pub async fn foo<S>() where S: T {
    S::bar().await;
    S::bar().await;
}
pub fn main() {}
