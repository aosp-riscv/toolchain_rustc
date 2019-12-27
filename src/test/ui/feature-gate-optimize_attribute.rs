#![crate_type="rlib"]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![optimize(speed)] //~ ERROR `#[optimize]` attribute is an unstable feature
=======
#![optimize(speed)] //~ ERROR the `#[optimize]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[optimize(size)] //~ ERROR `#[optimize]` attribute is an unstable feature
=======
#[optimize(size)] //~ ERROR the `#[optimize]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
mod module {

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[optimize(size)] //~ ERROR `#[optimize]` attribute is an unstable feature
=======
#[optimize(size)] //~ ERROR the `#[optimize]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn size() {}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[optimize(speed)] //~ ERROR `#[optimize]` attribute is an unstable feature
=======
#[optimize(speed)] //~ ERROR the `#[optimize]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn speed() {}

#[optimize(banana)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR `#[optimize]` attribute is an unstable feature
=======
//~^ ERROR the `#[optimize]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
//~| ERROR E0722
fn not_known() {}

}
