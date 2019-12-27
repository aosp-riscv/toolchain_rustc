// Regression test for #55219:
//
// The `Self::HASH_LEN` here expands to a "self-type" where `T` is not
// known. This unbound inference variable was causing an ICE.
//
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// check-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

pub struct Foo<T>(T);

impl<T> Foo<T> {
    const HASH_LEN: usize = 20;

    fn stuff() {
        let _ = Self::HASH_LEN;
    }
}

fn main() { }
