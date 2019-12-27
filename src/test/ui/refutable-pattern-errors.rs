// ignore-tidy-linelength

fn func((1, (Some(1), 2..=3)): (isize, (Option<isize>, isize))) { }
//~^ ERROR refutable pattern in function argument: `(_, _)` not covered

fn main() {
    let (1, (Some(1), 2..=3)) = (1, (None, 2));
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR refutable pattern in local binding: `(std::i32::MIN..=0i32, _)` not covered
=======
    //~^ ERROR refutable pattern in local binding: `(std::i32::MIN..=0i32, _)` and `(2i32..=std::i32::MAX, _)` not covered
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
