#![deny(unused_qualifications)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[allow(deprecated)]
=======
#![allow(deprecated)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

mod foo {
    pub fn bar() {}
}

fn main() {
    use foo::bar;
    foo::bar(); //~ ERROR: unnecessary qualification
    bar();

    let _ = || -> Result<(), ()> { try!(Ok(())); Ok(()) }; // issue #37345

    macro_rules! m { () => {
        $crate::foo::bar(); // issue #37357
        ::foo::bar(); // issue #38682
    } }
    m!();
}
