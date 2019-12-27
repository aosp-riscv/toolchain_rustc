// edition:2018

// Test that a feature gate is needed to use `impl Trait` as the
// return type of an async.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
trait Trait<'a, 'b> { }
impl<T> Trait<'_, '_> for T { }

async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
    //~^ ERROR ambiguous lifetime bound
    (a, b)
}

fn main() {
    let _ = async_ret_impl_trait(&22, &44);
}
