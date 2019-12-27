fn foo() -> isize {
    let x: isize;

    loop {
        break;
        x = 0;
    }

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    println!("{}", x); //~ ERROR borrow of possibly uninitialized variable: `x`
=======
    println!("{}", x); //~ ERROR borrow of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

    return 17;
}

fn main() { println!("{}", foo()); }
