trait Future {
    type Item;
    type Error;
}

use std::error::Error;

fn foo() -> impl Future<Item=(), Error=Box<dyn Error>> {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR missing lifetime
=======
//~^ ERROR not satisfied
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    Ok(())
}

fn main() {}
