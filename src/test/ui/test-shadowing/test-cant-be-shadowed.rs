<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// build-pass (FIXME(62277): could be check-pass?)
// aux-build:test_macro.rs
// compile-flags:--test

#[macro_use] extern crate test_macro;

#[test]
fn foo(){}

macro_rules! test { () => () }

#[test]
fn bar() {}
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
