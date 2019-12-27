// This test created a generator whose size was not rounded to a multiple of its
// alignment. This caused an assertion error in codegen.

// build-pass
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn noop() {}

async fn foo() {
    // This suspend should be the largest variant.
    {
        let x = [0u8; 17];
        noop().await;
        println!("{:?}", x);
    }

    // Add one variant that's aligned to 8 bytes.
    {
        let x = 0u64;
        noop().await;
        println!("{:?}", x);
    }
}

fn main() {
    let _ = foo();
}
