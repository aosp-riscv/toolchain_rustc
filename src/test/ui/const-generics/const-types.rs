// run-pass

#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete and may cause the compiler to crash

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[allow(dead_code)]
=======
#![allow(dead_code, unused_variables)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

struct ConstArray<T, const LEN: usize> {
    array: [T; LEN],
}

fn main() {
    let arr = ConstArray::<i32, 8> {
        array: [0; 8],
    };
}
