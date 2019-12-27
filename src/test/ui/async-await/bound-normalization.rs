// check-pass
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
// See issue 60414

trait Trait {
    type Assoc;
}

async fn foo<T: Trait<Assoc=()>>() -> T::Assoc {
    ()
}

fn main() {}
