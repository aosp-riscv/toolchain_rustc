<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// check-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

struct Slice(&'static [&'static [u8]]);

static MAP: Slice = Slice(&[
    b"CloseEvent" as &'static [u8],
]);

fn main() {}
