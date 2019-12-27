fn main() {
    &{[1, 2, 3][4]};
    //~^ ERROR index out of bounds
    //~| ERROR reaching this expression at runtime will panic or abort
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~| ERROR this expression will panic at runtime
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
