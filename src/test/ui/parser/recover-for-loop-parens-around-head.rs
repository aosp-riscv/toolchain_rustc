// Here we test that the parser is able to recover in a situation like
// `for ( $pat in $expr )` since that is familiar syntax in other languages.
// Instead we suggest that the user writes `for $pat in $expr`.

#![deny(unused)] // Make sure we don't trigger `unused_parens`.

fn main() {
    let vec = vec![1, 2, 3];

    for ( elem in vec ) {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        //~^ ERROR expected one of `)`, `,`, or `@`, found `in`
=======
        //~^ ERROR expected one of `)`, `,`, `@`, or `|`, found `in`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        //~| ERROR unexpected closing `)`
        const RECOVERY_WITNESS: () = 0; //~ ERROR mismatched types
    }
}
