// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
foo!(); //~ ERROR cannot find macro `foo!` in this scope

pub(in ::bar) struct Baz {} //~ ERROR cannot determine resolution for the visibility
=======
foo!(); //~ ERROR cannot find macro `foo` in this scope

pub(in ::bar) struct Baz {} //~ ERROR cannot determine resolution for the visibility

fn main() {}
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
