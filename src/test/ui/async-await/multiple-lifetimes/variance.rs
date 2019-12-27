// edition:2018
// run-pass

// Test for async fn where the parameters have distinct lifetime
// parameters that appear in all possible variances.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

#[allow(dead_code)]
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn lotsa_lifetimes<'a, 'b, 'c>(_: fn(&'a u8), _: fn(&'b u8) -> &'b u8, _: fn() -> &'c u8) { }

fn take_any(_: &u8) { }
fn identify(x: &u8) -> &u8 { x }
fn give_back() -> &'static u8 { &22 }

fn main() {
    let _ = lotsa_lifetimes(take_any, identify, give_back);
}
