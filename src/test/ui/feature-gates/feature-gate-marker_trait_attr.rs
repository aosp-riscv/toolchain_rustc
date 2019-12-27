use std::fmt::{Debug, Display};

#[marker] trait ExplicitMarker {}
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR marker traits is an experimental feature
=======
//~^ ERROR the `#[marker]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

impl<T: Display> ExplicitMarker for T {}
impl<T: Debug> ExplicitMarker for T {}

fn main() {}
