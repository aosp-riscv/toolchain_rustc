fn force<F>(f: F) where F: FnOnce() { f(); }
fn main() {
    let x: isize;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    force(|| {  //~ ERROR borrow of possibly uninitialized variable: `x`
=======
    force(|| {  //~ ERROR borrow of possibly-uninitialized variable: `x`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        println!("{}", x);
    });
}
