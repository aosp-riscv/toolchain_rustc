fn test1() {
    // In this test the outer 'a loop may terminate without `x` getting initialised. Although the
    // `x = loop { ... }` statement is reached, the value itself ends up never being computed and
    // thus leaving `x` uninit.
    let x: i32;
    'a: loop {
        x = loop { break 'a };
    }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    println!("{:?}", x); //~ ERROR borrow of possibly uninitialized variable
=======
    println!("{:?}", x); //~ ERROR borrow of possibly-uninitialized variable
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

// test2 and test3 should not fail.
fn test2() {
    // In this test the `'a` loop will never terminate thus making the use of `x` unreachable.
    let x: i32;
    'a: loop {
        x = loop { continue 'a };
    }
    println!("{:?}", x);
}

fn test3() {
    let x: i32;
    // Similarly, the use of variable `x` is unreachable.
    'a: loop {
        x = loop { return };
    }
    println!("{:?}", x);
}

fn main() {
}
