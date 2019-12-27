type A = rustfmt; //~ ERROR expected type, found tool module `rustfmt`
type B = rustfmt::skip; //~ ERROR expected type, found tool attribute `rustfmt::skip`

#[derive(rustfmt)] //~ ERROR cannot find derive macro `rustfmt` in this scope
struct S;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// Interpreted as a feature gated custom attribute
#[rustfmt] //~ ERROR cannot find attribute macro `rustfmt` in this scope
=======
// Interpreted as an unstable custom attribute
#[rustfmt] //~ ERROR cannot find attribute `rustfmt` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn check() {}

#[rustfmt::skip] // OK
fn main() {
    rustfmt; //~ ERROR expected value, found tool module `rustfmt`
    rustfmt!(); //~ ERROR cannot find macro `rustfmt` in this scope

    rustfmt::skip; //~ ERROR expected value, found tool attribute `rustfmt::skip`
}
