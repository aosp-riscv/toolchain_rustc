<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass

#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete and may cause the compiler to crash

use std::mem;

fn foo<const SIZE: usize>() {
    let arr: [u8; SIZE] = unsafe {
        #[allow(deprecated)]
        let mut array: [u8; SIZE] = mem::uninitialized();
=======
// check-pass

#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete and may cause the compiler to crash

use std::mem;

fn foo<const SIZE: usize>() {
    let arr: [u8; SIZE] = unsafe {
        #[allow(deprecated)]
        let array: [u8; SIZE] = mem::uninitialized();
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        array
    };
}

fn main() {}
