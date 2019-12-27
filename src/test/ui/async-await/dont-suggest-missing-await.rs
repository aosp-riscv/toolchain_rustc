// edition:2018

// This test ensures we don't make the suggestion in bodies that aren't `async`.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn take_u32(x: u32) {}

async fn make_u32() -> u32 {
    22
}

async fn dont_suggest_await_in_closure() {
    || {
        let x = make_u32();
        take_u32(x)
        //~^ ERROR mismatched types [E0308]
    };
}

fn main() {}
