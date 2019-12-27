// run-pass
#![allow(unused_attributes)]
// aux-build:iss.rs

// pretty-expanded FIXME #23616

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![crate_id="issue-6919"]
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
extern crate issue6919_3;

pub fn main() {
    let _ = issue6919_3::D.k;
}
