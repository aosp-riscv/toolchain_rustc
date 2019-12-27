// run-pass
fn main() {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    // -------- Simplified test case --------

    let _ = || 0..=1;

    // -------- Original test case --------

=======
    // Simplified test case
    let _ = || 0..=1;

    // Original test case
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    let full_length = 1024;
    let range = {
        // do some stuff, omit here
        None
    };

    let range = range.map(|(s, t)| s..=t).unwrap_or(0..=(full_length-1));

    assert_eq!(range, 0..=1023);
}
