<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// run-pass
=======
// check-pass
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

use std::ops::Deref;

trait MyTrait {
    fn do_something(&self);
    fn as_str(&self) -> &str;
}

impl Deref for dyn MyTrait {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

fn trait_object_does_something(t: &dyn MyTrait) {
    t.do_something()
}

fn main() {}
