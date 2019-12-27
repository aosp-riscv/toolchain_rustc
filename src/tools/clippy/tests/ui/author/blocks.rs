#![feature(stmt_expr_attributes)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
#![allow(redundant_semicolon)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

#[rustfmt::skip]
fn main() {
    #[clippy::author]
    {
        ;;;;
    }
}

#[clippy::author]
fn foo() {
    let x = 42i32;
    -x;
}
