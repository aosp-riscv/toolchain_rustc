// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(param_attrs)]

trait Trait2015 { fn foo(#[allow(C)] i32); }
//~^ ERROR expected one of `:` or `@`, found `)`
=======
trait Trait2015 { fn foo(#[allow(C)] i32); }
//~^ ERROR expected one of `:`, `@`, or `|`, found `)`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

fn main() {}
