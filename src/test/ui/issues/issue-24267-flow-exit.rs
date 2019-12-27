// Ensure that we reject code when a nonlocal exit (`break`,
// `continue`) causes us to pop over a needed assignment.

pub fn main() {
    foo1();
    foo2();
}

pub fn foo1() {
    let x: i32;
    loop { x = break; }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    println!("{}", x); //~ ERROR borrow of possibly uninitialized variable: `x`
=======
    println!("{}", x); //~ ERROR borrow of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

pub fn foo2() {
    let x: i32;
    for _ in 0..10 { x = continue; }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    println!("{}", x); //~ ERROR borrow of possibly uninitialized variable: `x`
=======
    println!("{}", x); //~ ERROR borrow of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
