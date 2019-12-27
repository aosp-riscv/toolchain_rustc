fn main() {
    let e: i32;
    match e {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        //~^ ERROR use of possibly uninitialized variable
=======
        //~^ ERROR use of possibly-uninitialized variable
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        ref u if true => {}
        ref v if true => {
            let tx = 0;
            &tx;
        }
        _ => (),
    }
}
