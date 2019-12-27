// build-pass (FIXME(62277): could be check-pass?)
// edition:2018
//
// Tests that we properly handle StorageDead/StorageLives for temporaries
// created in async loop bodies.

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn bar() -> Option<()> {
    Some(())
}

async fn listen() {
    while let Some(_) = bar().await {
        String::new();
    }
}

fn main() {
    listen();
}
