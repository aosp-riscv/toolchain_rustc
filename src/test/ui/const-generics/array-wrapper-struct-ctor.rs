// run-pass

#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete and may cause the compiler to crash

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
#![allow(dead_code)]

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct ArrayStruct<T, const N: usize> {
    data: [T; N],
}

struct ArrayTuple<T, const N: usize>([T; N]);

fn main() {
    let _ = ArrayStruct { data: [0u32; 8] };
    let _ = ArrayTuple([0u32; 8]);
}
