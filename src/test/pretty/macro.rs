// pp-exact

#![feature(decl_macro)]

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
macro mac { ($ arg : expr) => { $ arg + $ arg } }
=======
pub(crate) macro mac { ($ arg : expr) => { $ arg + $ arg } }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

fn main() { }
