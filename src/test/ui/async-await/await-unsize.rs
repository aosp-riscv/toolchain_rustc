// Regression test for #62312

// check-pass
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn make_boxed_object() -> Box<dyn Send> {
    Box::new(()) as _
}

async fn await_object() {
    let _ = make_boxed_object().await;
}

fn main() {}
