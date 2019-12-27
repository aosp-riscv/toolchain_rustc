// aux-build:test-macros.rs

#[macro_use]
extern crate test_macros;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[derive_Empty] //~ ERROR cannot find attribute macro `derive_Empty` in this scope
=======
#[derive_Empty] //~ ERROR cannot find attribute `derive_Empty` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct A;

fn main() {}
