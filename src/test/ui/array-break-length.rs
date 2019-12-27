fn main() {
    loop {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        |_: [_; break]| {} //~ ERROR: `break` outside of loop
=======
        |_: [_; break]| {} //~ ERROR: `break` outside of a loop
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        //~^ ERROR mismatched types
    }

    loop {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        |_: [_; continue]| {} //~ ERROR: `continue` outside of loop
=======
        |_: [_; continue]| {} //~ ERROR: `continue` outside of a loop
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        //~^ ERROR mismatched types
    }
}
