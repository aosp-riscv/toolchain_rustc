// aux-build:test-macros.rs

#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate test_macros;

// NB. these errors aren't the best errors right now, but they're definitely
// intended to be errors. Somehow using a custom attribute in these positions
// should either require a feature gate or not be allowed on stable.

fn _test6<#[empty_attr] T>() {}
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR: unknown to the compiler
=======
//~^ ERROR: expected an inert attribute, found an attribute macro
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

fn _test7() {
    match 1 {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        #[empty_attr] //~ ERROR: unknown to the compiler
=======
        #[empty_attr] //~ ERROR: expected an inert attribute, found an attribute macro
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        0 => {}
        _ => {}
    }
}

fn main() {
}
