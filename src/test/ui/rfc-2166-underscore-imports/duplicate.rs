<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// build-pass (FIXME(62277): could be check-pass?)
// aux-build:duplicate.rs

extern crate duplicate;

#[duplicate::duplicate]
use main as _; // OK

macro_rules! duplicate {
    ($item: item) => { $item $item }
}

duplicate!(use std as _;); // OK

fn main() {}
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
