<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// check-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete and may cause the compiler to crash

struct A<const N: usize>; // ok

fn main() {}
