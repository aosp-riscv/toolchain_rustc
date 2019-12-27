#![feature(decl_macro)]

mod foo {
    pub macro m() { Vec::new(); ().clone() }
    fn f() { ::bar::m!(); }
}

#[no_implicit_prelude]
mod bar {
    pub macro m() {
        Vec::new(); //~ ERROR failed to resolve
        ().clone() //~ ERROR no method named `clone` found
    }
    fn f() {
        ::foo::m!();
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        assert_eq!(0, 0); //~ ERROR cannot find macro `panic!` in this scope
=======
        assert_eq!(0, 0); //~ ERROR cannot find macro `panic` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }
}

fn main() {}
