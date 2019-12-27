struct A {
    banana: u8,
}

impl A {
    fn new(peach: u8) -> A {
        A {
            banana: banana //~ ERROR cannot find value `banana` in this scope
        }
    }

    fn foo(&self, peach: u8) -> A {
        A {
            banana: banana //~ ERROR cannot find value `banana` in this scope
        }
    }
}
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======

fn main() {}
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
