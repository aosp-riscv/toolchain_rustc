// Check that `<-` and `in` syntax gets a hard error.

fn foo() {
    let (x, y) = (0, 0);
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    x <- y; //~ ERROR expected one of
=======
    x <- y; //~ ERROR unexpected token: `<-`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

fn main() {
    let (foo, bar) = (0, 0);
    in(foo) { bar }; //~ ERROR expected expression, found keyword `in`
}
