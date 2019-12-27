// Test that `#[rustc_*]` attributes are gated by `rustc_attrs` feature gate.

#![feature(decl_macro)]

mod rustc { pub macro unknown() {} }
mod unknown { pub macro rustc() {} }

#[rustc::unknown]
//~^ ERROR attributes starting with `rustc` are reserved for use by the `rustc` compiler
//~| ERROR expected attribute, found macro `rustc::unknown`
fn f() {}

#[unknown::rustc]
//~^ ERROR attributes starting with `rustc` are reserved for use by the `rustc` compiler
//~| ERROR expected attribute, found macro `unknown::rustc`
fn g() {}

#[rustc_dummy]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR used by the test suite
#[rustc_unknown]
//~^ ERROR attributes starting with `rustc` are reserved for use by the `rustc` compiler
//~| ERROR cannot find attribute macro `rustc_unknown` in this scope
=======
//~^ ERROR the `#[rustc_dummy]` attribute is just used for rustc unit tests
#[rustc_unknown]
//~^ ERROR attributes starting with `rustc` are reserved for use by the `rustc` compiler
//~| ERROR cannot find attribute `rustc_unknown` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn main() {}
