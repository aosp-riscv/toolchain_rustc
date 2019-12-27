#![allow(incomplete_features)]
#![feature(generic_associated_types)]

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//FIXME(#44265): The lifetime shadowing and type parameter shadowing
// should cause an error. Now it compiles (erroneously) and this will be addressed
// by a future PR. Then remove the following:
// build-pass (FIXME(62277): could be check-pass?)

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
trait Shadow<'a> {
    //FIXME(#44265): The lifetime parameter shadowing should cause an error.
    type Bar<'a>;
}

trait NoShadow<'a> {
    type Bar<'b>; // OK
}

impl<'a> NoShadow<'a> for &'a u32 {
    //FIXME(#44265): The lifetime parameter shadowing should cause an error.
    type Bar<'a> = i32;
}

trait ShadowT<T> {
    type Bar<T>; //~ ERROR the name `T` is already used
}

trait NoShadowT<T> {
    type Bar<U>; // OK
}

impl<T> NoShadowT<T> for Option<T> {
    type Bar<T> = i32; //~ ERROR the name `T` is already used
}

fn main() {}
