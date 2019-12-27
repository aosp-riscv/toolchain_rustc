// edition:2018
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)

#![feature(async_await)]
=======
// ignore-x86
// ^ due to stderr output differences
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

async fn print_dur() {}

fn main() {
    async { let (); }.await;
    //~^ ERROR `await` is only allowed inside `async` functions and blocks
    async {
    //~^ ERROR `await` is only allowed inside `async` functions and blocks
        let task1 = print_dur().await;
    }.await;
    (|_| 2333).await;
    //~^ ERROR `await` is only allowed inside `async` functions and blocks
    //~^^ ERROR
}
