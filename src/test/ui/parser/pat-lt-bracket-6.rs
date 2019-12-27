fn main() {
    struct Test(&'static u8, [u8; 0]);
    let x = Test(&0, []);

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    let Test(&desc[..]) = x; //~ ERROR: expected one of `)`, `,`, or `@`, found `[`
    //~^ ERROR subslice patterns are unstable
=======
    let Test(&desc[..]) = x;
    //~^ ERROR: expected one of `)`, `,`, `@`, or `|`, found `[`
    //~^^ ERROR subslice patterns are unstable
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

const RECOVERY_WITNESS: () = 0; //~ ERROR mismatched types
