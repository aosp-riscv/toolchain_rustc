//! Test for Clippy lint renames.
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)

=======
// run-rustfix

#![allow(dead_code)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
// allow the new lint name here, to test if the new name works
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::new_without_default)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::redundant_static_lifetimes)]
// warn for the old lint name here, to test if the renaming worked
#![warn(clippy::cyclomatic_complexity)]

#[warn(clippy::stutter)]
fn main() {}

#[warn(clippy::new_without_default_derive)]
struct Foo;

#[warn(clippy::const_static_lifetime)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
static Bar: &'static str = "baz";

impl Foo {
    fn new() -> Self {
        Foo
    }
}
=======
fn foo() {}
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
