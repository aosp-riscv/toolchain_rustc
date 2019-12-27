// run-pass

#![allow(irrefutable_let_patterns)]

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use std::ops::Range;

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn main() {
    let x: bool;
    // This should associate as: `(x = (true && false));`.
    x = true && false;
    assert!(!x);

    fn _f1() -> bool {
        // Should associate as `(let _ = (return (true && false)))`.
        if let _ = return true && false {};
    }
    assert!(!_f1());
}
