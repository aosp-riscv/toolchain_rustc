// edition:2018
// run-rustfix

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn take_u32(_x: u32) {}

async fn make_u32() -> u32 {
    22
}

#[allow(unused)]
async fn suggest_await_in_async_fn() {
    let x = make_u32();
    take_u32(x)
    //~^ ERROR mismatched types [E0308]
    //~| HELP consider using `.await` here
    //~| SUGGESTION x.await
}

fn main() {}
