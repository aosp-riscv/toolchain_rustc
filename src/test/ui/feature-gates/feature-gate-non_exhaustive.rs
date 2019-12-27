//#![feature(non_exhaustive)]

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[non_exhaustive] //~ERROR non exhaustive is an experimental feature
=======
#[non_exhaustive] //~ERROR the `#[non_exhaustive]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
pub enum NonExhaustiveEnum {
    Unit,
    Tuple(u32),
    Struct { field: u32 }
}

fn main() { }
