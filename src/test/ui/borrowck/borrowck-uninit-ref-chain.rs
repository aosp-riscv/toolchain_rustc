struct S<X, Y> {
    x: X,
    y: Y,
}

fn main() {
    let x: &&Box<i32>;
    let _y = &**x; //~ [E0381]

    let x: &&S<i32, i32>;
    let _y = &**x; //~ [E0381]

    let x: &&i32;
    let _y = &**x; //~ [E0381]


    let mut a: S<i32, i32>;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    a.x = 0;            //~ ERROR assign to part of possibly uninitialized variable: `a` [E0381]
=======
    a.x = 0;            //~ ERROR assign to part of possibly-uninitialized variable: `a` [E0381]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    let _b = &a.x;

    let mut a: S<&&i32, &&i32>;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    a.x = &&0;          //~ ERROR assign to part of possibly uninitialized variable: `a` [E0381]
=======
    a.x = &&0;          //~ ERROR assign to part of possibly-uninitialized variable: `a` [E0381]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    let _b = &**a.x;


    let mut a: S<i32, i32>;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    a.x = 0;            //~ ERROR assign to part of possibly uninitialized variable: `a` [E0381]
=======
    a.x = 0;            //~ ERROR assign to part of possibly-uninitialized variable: `a` [E0381]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    let _b = &a.y;

    let mut a: S<&&i32, &&i32>;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    a.x = &&0;          //~ assign to part of possibly uninitialized variable: `a` [E0381]
=======
    a.x = &&0;          //~ assign to part of possibly-uninitialized variable: `a` [E0381]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    let _b = &**a.y;
}
