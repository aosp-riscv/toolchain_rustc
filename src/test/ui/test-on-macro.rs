<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// build-pass (FIXME(62277): could be check-pass?)
// compile-flags:--test

#![deny(warnings)]

macro_rules! foo {
    () => (fn foo(){})
}

#[test]
foo!();

fn main(){}
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
