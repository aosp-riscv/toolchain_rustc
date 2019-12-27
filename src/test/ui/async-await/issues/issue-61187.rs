// edition:2018
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

fn main() {
}
=======

fn main() {}
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

async fn response(data: Vec<u8>) {
    data.reverse(); //~ ERROR E0596
}
