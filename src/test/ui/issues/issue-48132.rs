// Regression test for #48132. This was failing due to problems around
// the projection caching and dropck type enumeration.

// run-pass

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
#![allow(dead_code)]

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct Inner<I, V> {
    iterator: I,
    item: V,
}

struct Outer<I: Iterator> {
    inner: Inner<I, I::Item>,
}

fn outer<I>(iterator: I) -> Outer<I>
where I: Iterator,
      I::Item: Default,
{
    Outer {
        inner: Inner {
            iterator: iterator,
            item: Default::default(),
        }
    }
}

fn main() {
    outer(std::iter::once(&1).cloned());
}
