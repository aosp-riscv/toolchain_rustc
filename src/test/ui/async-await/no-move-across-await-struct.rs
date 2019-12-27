// compile-fail
// edition:2018
// compile-flags: --crate-type lib

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn no_move_across_await_struct() -> Vec<usize> {
    let s = Small { x: vec![31], y: vec![19, 1441] };
    needs_vec(s.x).await;
    s.x
    //~^ ERROR use of moved value: `s.x`
}

struct Small {
    x: Vec<usize>,
    y: Vec<usize>,
}

async fn needs_vec(_vec: Vec<usize>) {}
