<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// edition:2018

struct S;

impl S {
    async fn foo() {} //~ ERROR async fn is unstable
}

trait T {
    async fn foo(); //~ ERROR trait fns cannot be declared `async`
    //~^ ERROR async fn is unstable
}

async fn foo() {} //~ ERROR async fn is unstable

fn main() {
    let _ = async {}; //~ ERROR async blocks are unstable
}
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
