// edition:2018
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

pub async fn f(x: Option<usize>) {
    x.take();
    //~^ ERROR cannot borrow `x` as mutable, as it is not declared as mutable [E0596]
}

pub async fn g(x: usize) {
    x += 1;
    //~^ ERROR cannot assign twice to immutable variable `x` [E0384]
}

fn main() {}
