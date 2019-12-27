// Check that unknown attribute error is shown even if there are unresolved macros.

#[marco_use] // typo
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR cannot find attribute macro `marco_use` in this scope
=======
//~^ ERROR cannot find attribute `marco_use` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
mod foo {
    macro_rules! bar {
        () => ();
    }
}

fn main() {
   bar!(); //~ ERROR cannot find macro `bar` in this scope
}
