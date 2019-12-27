#[rustc_attribute_should_be_reserved]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR cannot find attribute macro `rustc_attribute_should_be_reserved` in this scope
=======
//~^ ERROR cannot find attribute `rustc_attribute_should_be_reserved` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
//~| ERROR attributes starting with `rustc` are reserved for use by the `rustc` compiler

macro_rules! foo {
    () => (());
}

fn main() {
    foo!(); //~ ERROR cannot determine resolution for the macro `foo`
}
