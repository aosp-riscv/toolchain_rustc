// Check that mutable promoted length zero arrays don't check for conflicting
// access

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// check-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

pub fn main() {
    let mut x: Vec<&[i32; 0]> = Vec::new();
    for _ in 0..10 {
        x.push(&[]);
    }
}
