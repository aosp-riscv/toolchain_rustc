// edition:2018
// run-pass

// Test that we can use async fns with multiple arbitrary lifetimes.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8, _: fn(&u8)) {}

fn gimme(_: &u8) { }

fn main() {
    let _ = multiple_named_lifetimes(&22, &44, gimme);
}
