// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct S;

impl S {
    async unsafe fn f() {}
}

async unsafe fn f() {}

async fn g() {
    S::f(); //~ ERROR call to unsafe function is unsafe
    f(); //~ ERROR call to unsafe function is unsafe
}

fn main() {
    S::f(); //~ ERROR call to unsafe function is unsafe
    f(); //~ ERROR call to unsafe function is unsafe
}
