// compile-fail
// edition:2018
// compile-flags: --crate-type lib

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn no_move_across_await_tuple() -> Vec<usize> {
    let x = (vec![3], vec![4, 4]);
    drop(x.1);
    nothing().await;
    x.1
    //~^ ERROR use of moved value: `x.1`
}

async fn nothing() {}
