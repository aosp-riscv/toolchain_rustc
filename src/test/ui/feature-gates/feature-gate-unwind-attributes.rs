// compile-flags: -C no-prepopulate-passes -Cpasses=name-anon-globals

#![crate_type = "lib"]

extern {
// CHECK: Function Attrs: nounwind
// CHECK-NEXT: declare void @extern_fn
    fn extern_fn();
// CHECK-NOT: Function Attrs: nounwind
// CHECK: declare void @unwinding_extern_fn
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    #[unwind(allowed)] //~ ERROR `#[unwind]` is experimental
=======
    #[unwind(allowed)] //~ ERROR the `#[unwind]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    fn unwinding_extern_fn();
}

pub unsafe fn force_declare() {
    extern_fn();
    unwinding_extern_fn();
}
