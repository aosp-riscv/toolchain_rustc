// ignore-s390x
// ignore-emscripten
// ignore-powerpc
// ignore-powerpc64
// ignore-powerpc64le
// ignore-sparc
// ignore-sparc64
// ignore-mips
// ignore-mips64

#![feature(asm)]

fn foo(x: isize) { println!("{}", x); }

#[cfg(any(target_arch = "x86",
          target_arch = "x86_64",
          target_arch = "arm",
          target_arch = "aarch64"))]
pub fn main() {
    let x: isize;
    unsafe {
        asm!("mov $1, $0" : "=r"(x) : "r"(x));
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        //~^ ERROR use of possibly uninitialized variable: `x`
=======
        //~^ ERROR use of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }
    foo(x);
}

#[cfg(not(any(target_arch = "x86",
              target_arch = "x86_64",
              target_arch = "arm",
              target_arch = "aarch64")))]
pub fn main() {}
