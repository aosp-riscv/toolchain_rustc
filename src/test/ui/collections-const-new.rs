<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass

#![allow(dead_code)]
// Test several functions can be used for constants
// 1. Vec::new()
// 2. String::new()

#![feature(const_vec_new)]
#![feature(const_string_new)]

const MY_VEC: Vec<usize> = Vec::new();

const MY_STRING: String = String::new();

pub fn main() {}
=======
// check-pass

// Test several functions can be used for constants
// 1. Vec::new()
// 2. String::new()

const MY_VEC: Vec<usize> = Vec::new();

const MY_STRING: String = String::new();

fn main() {}
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
