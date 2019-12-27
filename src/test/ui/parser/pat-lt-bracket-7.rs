fn main() {
    struct Thing(u8, [u8; 0]);
    let foo = core::iter::empty();

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    for Thing(x[]) in foo {} //~ ERROR: expected one of `)`, `,`, or `@`, found `[`
=======
    for Thing(x[]) in foo {}
    //~^ ERROR: expected one of `)`, `,`, `@`, or `|`, found `[`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

const RECOVERY_WITNESS: () = 0; //~ ERROR mismatched types
