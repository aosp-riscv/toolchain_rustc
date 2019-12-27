// Regression test for #55183: check a case where the self type from
// the inherent impl requires normalization to be equal to the
// user-provided type.
//
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// check-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

trait Mirror {
    type Me;
}

impl<T> Mirror for T {
    type Me = T;
}

struct Foo<A, B>(A, B);

impl<A> Foo<A, <A as Mirror>::Me> {
    fn m(_: A) { }
}

fn main() {
    <Foo<&'static u32, &u32>>::m(&22);
}
