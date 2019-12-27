<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass

#![feature(type_alias_impl_trait)]

type X = impl Clone;

fn bar<F: Fn(&i32) + Clone>(f: F) -> F {
    f
}

fn foo() -> X {
    bar(|x| ())
=======
// check-pass

#![feature(type_alias_impl_trait)]

type X = impl Clone;

fn bar<F: Fn(&i32) + Clone>(f: F) -> F {
    f
}

fn foo() -> X {
    bar(|_| ())
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

fn main() {}
