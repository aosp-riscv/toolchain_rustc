// edition:2018
// run-pass

// Test that we can use async fns with multiple arbitrary lifetimes.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}

fn main() {
    let _ = multiple_elided_lifetimes(&22, &44);
}
