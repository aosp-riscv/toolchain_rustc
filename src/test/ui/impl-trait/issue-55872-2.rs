// edition:2018
// ignore-tidy-linelength
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await, type_alias_impl_trait)]
=======

#![feature(type_alias_impl_trait)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

pub trait Bar {
    type E: Copy;

    fn foo<T>() -> Self::E;
}

impl<S> Bar for S {
    type E = impl Copy;
    //~^ ERROR the trait bound `impl std::future::Future: std::marker::Copy` is not satisfied [E0277]
    fn foo<T>() -> Self::E {
    //~^ ERROR type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
        async {}
    }
}

fn main() {}
