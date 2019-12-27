enum Enum<'a> {
    A(&'a isize),
    B(bool),
}

fn foo() -> isize {
    let mut n = 42;
    let mut x = Enum::A(&mut n);
    match x {
        Enum::A(_) if { x = Enum::B(false); false } => 1,
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        //~^ ERROR cannot assign in a pattern guard
        //~| ERROR cannot assign `x` in match guard
=======
        //~^ ERROR cannot assign `x` in match guard
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        Enum::A(_) if { let y = &mut x; *y = Enum::B(false); false } => 1,
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        //~^ ERROR cannot mutably borrow in a pattern guard
        //~| ERROR cannot assign in a pattern guard
        //~| ERROR cannot mutably borrow `x` in match guard
=======
        //~^ ERROR cannot mutably borrow `x` in match guard
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        Enum::A(p) => *p,
        Enum::B(_) => 2,
    }
}

fn main() {
    foo();
}
