<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(bind_by_move_pattern_guards)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct A { a: Box<i32> }

fn foo(n: i32) {
    let x = A { a: Box::new(n) };
    let _y = match x {
        A { a: v } if { drop(v); true } => v,
        //~^ ERROR cannot move out of `v` in pattern guard
        _ => Box::new(0),
    };
}

fn main() {
    foo(107);
}
