<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[deprcated] //~ ERROR cannot find attribute macro `deprcated` in this scope
=======
#[deprcated] //~ ERROR cannot find attribute `deprcated` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn foo() {}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[tests] //~ ERROR cannot find attribute macro `tests` in this scope
=======
#[tests] //~ ERROR cannot find attribute `tests` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn bar() {}

#[rustc_err]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR cannot find attribute macro `rustc_err` in this scope
=======
//~^ ERROR cannot find attribute `rustc_err` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
//~| ERROR attributes starting with `rustc` are reserved for use by the `rustc` compiler

fn main() {}
