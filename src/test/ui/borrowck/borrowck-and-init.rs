fn main() {
    let i: isize;

    println!("{}", false && { i = 5; true });
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    println!("{}", i); //~ ERROR borrow of possibly uninitialized variable: `i`
=======
    println!("{}", i); //~ ERROR borrow of possibly-uninitialized variable: `i`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
