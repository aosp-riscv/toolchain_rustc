struct R<'a> {
    r: &'a R<'a>,
}

fn foo(res: Result<u32, &R>) -> u32 {
    let Ok(x) = res;
    //~^ ERROR refutable pattern
    x
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR use of possibly uninitialized variable: `x`
=======
    //~^ ERROR use of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

fn main() {
    foo(Ok(23));
}
