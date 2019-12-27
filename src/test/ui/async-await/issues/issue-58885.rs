// build-pass (FIXME(62277): could be check-pass?)
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct Xyz {
    a: u64,
}

trait Foo {}

impl Xyz {
    async fn do_sth<'a>(
        &'a self, foo: &'a dyn Foo
    ) -> bool
    {
        true
    }
}

fn main() {}
