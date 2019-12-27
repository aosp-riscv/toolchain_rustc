<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// #[test] attribute is not allowed on associated functions or methods
// reworded error message
// compile-flags:--test

struct A {}

impl A {
    #[test]
    fn new() -> A { //~ ERROR `#[test]` attribute is only allowed on non associated functions
        A {}
    }
}

#[test]
fn test() {
    let _ = A::new();
}

fn main() {}
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
