// Adaptation of existing ui test (from way back in
// rust-lang/rust#2329), that starts passing with this feature in
// place.

// run-pass
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)

#![feature(bind_by_move_pattern_guards)]
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();
    let x = Some(rx);
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    tx.send(false);
    tx.send(false);
=======
    tx.send(false).unwrap();
    tx.send(false).unwrap();
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    match x {
        Some(z) if z.recv().unwrap() => { panic!() },
        Some(z) => { assert!(!z.recv().unwrap()); },
        None => panic!()
    }
}
