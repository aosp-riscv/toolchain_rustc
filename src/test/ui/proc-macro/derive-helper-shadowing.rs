// aux-build:test-macros.rs

#[macro_use]
extern crate test_macros;

use test_macros::empty_attr as empty_helper;

#[empty_helper] //~ ERROR `empty_helper` is ambiguous
#[derive(Empty)]
struct S {
    // FIXME No ambiguity, attributes in non-macro positions are not resolved properly
    #[empty_helper]
    field: [u8; {
        // FIXME No ambiguity, derive helpers are not put into scope for non-attributes
        use empty_helper;

        // FIXME No ambiguity, derive helpers are not put into scope for inner items
        #[empty_helper]
        struct U;

        mod inner {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
            #[empty_helper] //~ ERROR cannot find attribute macro `empty_helper` in this scope
=======
            // FIXME No ambiguity, attributes in non-macro positions are not resolved properly
            #[empty_helper]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
            struct V;
        }

        0
    }]
}

fn main() {
    let s = S { field: [] };
}
