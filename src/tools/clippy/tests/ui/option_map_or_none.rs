// run-rustfix

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
#![allow(clippy::option_and_then_some)]

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn main() {
    let opt = Some(1);

    // Check `OPTION_MAP_OR_NONE`.
    // Single line case.
    let _ = opt.map_or(None, |x| Some(x + 1));
    // Multi-line case.
    #[rustfmt::skip]
    let _ = opt.map_or(None, |x| {
                        Some(x + 1)
                       });
}
