#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let mut origin: Point;
    origin = Point { x: 10, ..origin };
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR use of possibly uninitialized variable: `origin` [E0381]
=======
    //~^ ERROR use of possibly-uninitialized variable: `origin` [E0381]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    origin.clone();
}
