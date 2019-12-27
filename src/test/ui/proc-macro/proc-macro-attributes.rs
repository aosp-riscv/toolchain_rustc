// aux-build:derive-b.rs

#[macro_use]
extern crate derive_b;

#[B] //~ ERROR `B` is ambiguous
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[C] //~ ERROR cannot find attribute macro `C` in this scope
=======
#[C] //~ ERROR cannot find attribute `C` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[B(D)] //~ ERROR `B` is ambiguous
#[B(E = "foo")] //~ ERROR `B` is ambiguous
#[B(arbitrary tokens)] //~ ERROR `B` is ambiguous
#[derive(B)]
struct B;

fn main() {}
