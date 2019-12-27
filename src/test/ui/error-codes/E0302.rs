<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
fn main() {
    match Some(()) {
        None => { },
        option if { option = None; false } => { }, //~ ERROR E0302
        //~^ ERROR cannot assign to `option`, as it is immutable for the pattern guard
        Some(_) => { }
    }
}
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
