<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// check-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

enum Nat {
    S(Box<Nat>),
    Z
}
fn test(x: &mut Nat) {
    let mut p = &mut *x;
    loop {
        match p {
            &mut Nat::Z => break,
            &mut Nat::S(ref mut n) => p = &mut *n
        }
    }
}

fn main() {}
