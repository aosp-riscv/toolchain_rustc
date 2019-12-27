// compile-fail
// edition:2018
// compile-flags: --crate-type lib

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

async fn no_non_guaranteed_initialization(x: usize) -> usize {
    let y;
    if x > 5 {
        y = echo(10).await;
    }
    y
    //~^ use of possibly uninitialized variable: `y`
=======
async fn no_non_guaranteed_initialization(x: usize) -> usize {
    let y;
    if x > 5 {
        y = echo(10).await;
    }
    y
    //~^ use of possibly-uninitialized variable: `y`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

async fn echo(x: usize) -> usize { x + 1 }
