// build-pass (FIXME(62277): could be check-pass?)
// compile-flags: -Wunused

// ensure there are no special warnings about uninhabited types
// when deriving Debug on an empty enum

#[derive(Debug)]
enum Void {}

#[derive(Debug)]
enum Foo {
    Bar(u8),
    Void(Void), //~ WARN never used
}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
fn main() {}
=======
fn main() {
    let x = Foo::Bar(42);
    println!("{:?}", x);
}
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
