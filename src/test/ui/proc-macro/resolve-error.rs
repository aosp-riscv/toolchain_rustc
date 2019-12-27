// aux-build:derive-foo.rs
// aux-build:derive-clona.rs
// aux-build:test-macros.rs

#[macro_use]
extern crate derive_foo;
#[macro_use]
extern crate derive_clona;
extern crate test_macros;

use test_macros::empty as bang_proc_macro;
use test_macros::empty_attr as attr_proc_macro;

macro_rules! FooWithLongNam {
    () => {}
}

macro_rules! attr_proc_mac {
    () => {}
}

#[derive(FooWithLongNan)]
//~^ ERROR cannot find
struct Foo;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// Interpreted as a feature gated custom attribute
#[attr_proc_macra] //~ ERROR cannot find attribute macro `attr_proc_macra` in this scope
=======
// Interpreted as an unstable custom attribute
#[attr_proc_macra] //~ ERROR cannot find attribute `attr_proc_macra` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct Bar;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// Interpreted as a feature gated custom attribute
#[FooWithLongNan] //~ ERROR cannot find attribute macro `FooWithLongNan` in this scope
=======
// Interpreted as an unstable custom attribute
#[FooWithLongNan] //~ ERROR cannot find attribute `FooWithLongNan` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct Asdf;

#[derive(Dlone)]
//~^ ERROR cannot find
struct A;

#[derive(Dlona)]
//~^ ERROR cannot find
struct B;

#[derive(attr_proc_macra)]
//~^ ERROR cannot find
struct C;

fn main() {
    FooWithLongNama!();
    //~^ ERROR cannot find

    attr_proc_macra!();
    //~^ ERROR cannot find

    Dlona!();
    //~^ ERROR cannot find

    bang_proc_macrp!();
    //~^ ERROR cannot find
}
