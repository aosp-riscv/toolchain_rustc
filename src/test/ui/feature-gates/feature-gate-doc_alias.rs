<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[doc(alias = "foo")] //~ ERROR: `#[doc(alias = "...")]` is experimental
=======
#[doc(alias = "foo")] //~ ERROR: `#[doc(alias)]` is experimental
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
pub struct Foo;

fn main() {}
