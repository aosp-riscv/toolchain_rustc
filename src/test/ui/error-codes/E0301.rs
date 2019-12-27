<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
fn main() {
    match Some(()) {
        None => { },
        option if option.take().is_none() => {}, //~ ERROR E0301
        Some(_) => { } //~^ ERROR E0596
    }
}
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
