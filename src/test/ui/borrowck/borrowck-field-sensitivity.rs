#![feature(box_syntax)]

struct A { a: isize, b: Box<isize> }

fn deref_after_move() {
    let x = A { a: 1, b: box 2 };
    drop(x.b);
    drop(*x.b); //~ ERROR use of moved value: `x.b`
}

fn deref_after_fu_move() {
    let x = A { a: 1, b: box 2 };
    let y = A { a: 3, .. x };
    drop(*x.b); //~ ERROR use of moved value: `x.b`
}

fn borrow_after_move() {
    let x = A { a: 1, b: box 2 };
    drop(x.b);
    let p = &x.b; //~ ERROR borrow of moved value: `x.b`
    drop(**p);
}

fn borrow_after_fu_move() {
    let x = A { a: 1, b: box 2 };
    let _y = A { a: 3, .. x };
    let p = &x.b; //~ ERROR borrow of moved value: `x.b`
    drop(**p);
}

fn move_after_borrow() {
    let x = A { a: 1, b: box 2 };
    let p = &x.b;
    drop(x.b); //~ ERROR cannot move out of `x.b` because it is borrowed
    drop(**p);
}

fn fu_move_after_borrow() {
    let x = A { a: 1, b: box 2 };
    let p = &x.b;
    let _y = A { a: 3, .. x }; //~ ERROR cannot move out of `x.b` because it is borrowed
    drop(**p);
}

fn mut_borrow_after_mut_borrow() {
    let mut x = A { a: 1, b: box 2 };
    let p = &mut x.a;
    let q = &mut x.a; //~ ERROR cannot borrow `x.a` as mutable more than once at a time
    drop(*p);
    drop(*q);
}

fn move_after_move() {
    let x = A { a: 1, b: box 2 };
    drop(x.b);
    drop(x.b);  //~ ERROR use of moved value: `x.b`
}

fn move_after_fu_move() {
    let x = A { a: 1, b: box 2 };
    let _y = A { a: 3, .. x };
    drop(x.b);  //~ ERROR use of moved value: `x.b`
}

fn fu_move_after_move() {
    let x = A { a: 1, b: box 2 };
    drop(x.b);
    let _z = A { a: 3, .. x };  //~ ERROR use of moved value: `x.b`
}

fn fu_move_after_fu_move() {
    let x = A { a: 1, b: box 2 };
    let _y = A { a: 3, .. x };
    let _z = A { a: 4, .. x };  //~ ERROR use of moved value: `x.b`
}

// The following functions aren't yet accepted, but they should be.

fn copy_after_field_assign_after_uninit() {
    let mut x: A;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    x.a = 1; //~ ERROR assign to part of possibly uninitialized variable: `x`
=======
    x.a = 1; //~ ERROR assign to part of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    drop(x.a);
}

fn borrow_after_field_assign_after_uninit() {
    let mut x: A;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    x.a = 1; //~ ERROR assign to part of possibly uninitialized variable: `x`
=======
    x.a = 1; //~ ERROR assign to part of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    let p = &x.a;
    drop(*p);
}

fn move_after_field_assign_after_uninit() {
    let mut x: A;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    x.b = box 1; //~ ERROR assign to part of possibly uninitialized variable: `x`
=======
    x.b = box 1; //~ ERROR assign to part of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    drop(x.b);
}

fn main() {
    deref_after_move();
    deref_after_fu_move();

    borrow_after_move();
    borrow_after_fu_move();
    move_after_borrow();
    fu_move_after_borrow();
    mut_borrow_after_mut_borrow();

    move_after_move();
    move_after_fu_move();
    fu_move_after_move();
    fu_move_after_fu_move();

    copy_after_field_assign_after_uninit();
    borrow_after_field_assign_after_uninit();
    move_after_field_assign_after_uninit();
}
