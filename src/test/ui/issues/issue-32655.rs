macro_rules! foo (
    () => (
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        #[derive_Clone] //~ ERROR cannot find attribute macro `derive_Clone` in this scope
=======
        #[derive_Clone] //~ ERROR cannot find attribute `derive_Clone` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        struct T;
    );
);

macro_rules! bar (
    ($e:item) => ($e)
);

foo!();

bar!(
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    #[derive_Clone] //~ ERROR cannot find attribute macro `derive_Clone` in this scope
=======
    #[derive_Clone] //~ ERROR cannot find attribute `derive_Clone` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    struct S;
);

fn main() {}
