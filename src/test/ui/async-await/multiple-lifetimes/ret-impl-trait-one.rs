// edition:2018

// Test that a feature gate is needed to use `impl Trait` as the
// return type of an async.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await, member_constraints)]
=======
#![feature(member_constraints)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

trait Trait<'a> { }
impl<T> Trait<'_> for T { }

// Only `'a` permitted in return type, not `'b`.
async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
    //~^ ERROR lifetime mismatch
    (a, b)
}

// As above, but `'b: 'a`, so return type can be inferred to `(&'a u8,
// &'a u8)`.
async fn async_ret_impl_trait2<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a>
where
    'b: 'a,
{
    (a, b)
}

fn main() {
}
