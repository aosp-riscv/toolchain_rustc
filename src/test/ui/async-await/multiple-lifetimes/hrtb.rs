// edition:2018
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass

// Test that we can use async fns with multiple arbitrary lifetimes.

#![feature(async_await)]
#![allow(dead_code)]
=======
// check-pass

// Test that we can use async fns with multiple arbitrary lifetimes.
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

use std::ops::Add;

async fn multiple_hrtb_and_single_named_lifetime_ok<'c>(
    _: impl for<'a> Add<&'a u8>,
    _: impl for<'b> Add<&'b u8>,
    _: &'c u8,
) {}

fn main() {}
