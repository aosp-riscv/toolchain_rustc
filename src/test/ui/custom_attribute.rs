#![feature(stmt_expr_attributes)]

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[foo] //~ ERROR cannot find attribute macro `foo` in this scope
=======
#[foo] //~ ERROR cannot find attribute `foo` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn main() {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    #[foo] //~ ERROR cannot find attribute macro `foo` in this scope
=======
    #[foo] //~ ERROR cannot find attribute `foo` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    let x = ();
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    #[foo] //~ ERROR cannot find attribute macro `foo` in this scope
=======
    #[foo] //~ ERROR cannot find attribute `foo` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    x
}
