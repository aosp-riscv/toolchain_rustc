macro_rules! foo {
    () => {
        #[cfg_attr(all(), unknown)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        //~^ ERROR cannot find attribute macro `unknown` in this scope
=======
        //~^ ERROR cannot find attribute `unknown` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        fn foo() {}
    }
}

foo!();

fn main() {}
