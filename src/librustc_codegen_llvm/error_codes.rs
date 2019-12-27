<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
register_long_diagnostics! {
=======
register_diagnostics! {
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

E0511: r##"
Invalid monomorphization of an intrinsic function was used. Erroneous code
example:

```ignore (error-emitted-at-codegen-which-cannot-be-handled-by-compile_fail)
#![feature(platform_intrinsics)]

extern "platform-intrinsic" {
    fn simd_add<T>(a: T, b: T) -> T;
}

fn main() {
    unsafe { simd_add(0, 1); }
    // error: invalid monomorphization of `simd_add` intrinsic
}
```

The generic type has to be a SIMD type. Example:

```
#![feature(repr_simd)]
#![feature(platform_intrinsics)]

#[repr(simd)]
#[derive(Copy, Clone)]
struct i32x2(i32, i32);

extern "platform-intrinsic" {
    fn simd_add<T>(a: T, b: T) -> T;
}

unsafe { simd_add(i32x2(0, 0), i32x2(1, 2)); } // ok!
```
"##,

}
