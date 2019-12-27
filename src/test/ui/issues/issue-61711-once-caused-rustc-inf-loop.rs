// Issue 61711: A crate pub re-exporting `crate` was causing an
// infinite loop.

// edition:2018
// aux-build:xcrate-issue-61711-b.rs
// compile-flags:--extern xcrate_issue_61711_b

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// build-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

fn f<F: Fn(xcrate_issue_61711_b::Struct)>(_: F) { }
fn main() { }
