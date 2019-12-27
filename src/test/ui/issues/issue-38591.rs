<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// check-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

struct S<T> {
    t : T,
    s : Box<S<fn(u : T)>>
}

fn f(x : S<u32>) {}

fn main () {}
