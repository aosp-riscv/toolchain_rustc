fn test(cond: bool) {
    let v;
    while cond {
        v = 3;
        break;
    }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    println!("{}", v); //~ ERROR borrow of possibly uninitialized variable: `v`
=======
    println!("{}", v); //~ ERROR borrow of possibly-uninitialized variable: `v`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

fn main() {
    test(true);
}
