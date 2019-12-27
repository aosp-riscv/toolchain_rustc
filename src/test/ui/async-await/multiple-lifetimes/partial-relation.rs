// edition:2018
// run-pass

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
async fn lotsa_lifetimes<'a, 'b, 'c>(a: &'a u32, b: &'b u32, c: &'c u32) -> (&'a u32, &'b u32)
    where 'b: 'a
{
    drop((a, c));
    (b, b)
}

fn main() {
    let _ = lotsa_lifetimes(&22, &44, &66);
}
