fn main() {
    |_: [_; continue]| {}; //~ ERROR: `continue` outside of a loop

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    while |_: [_; continue]| {} {} //~ ERROR: `continue` outside of loop
=======
    while |_: [_; continue]| {} {} //~ ERROR: `continue` outside of a loop
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    //~^ ERROR mismatched types

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    while |_: [_; break]| {} {} //~ ERROR: `break` outside of loop
=======
    while |_: [_; break]| {} {} //~ ERROR: `break` outside of a loop
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    //~^ ERROR mismatched types
}
