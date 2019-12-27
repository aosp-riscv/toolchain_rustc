// aux-build:two_macros.rs

#![feature(macro_reexport)] //~ ERROR feature has been removed

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[macro_reexport(macro_one)] //~ ERROR cannot find attribute macro `macro_reexport` in this scope
=======
#[macro_reexport(macro_one)] //~ ERROR cannot find attribute `macro_reexport` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
extern crate two_macros;

fn main() {}
