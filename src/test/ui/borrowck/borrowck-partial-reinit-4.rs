struct Test;

struct Test2(Option<Test>);

impl Drop for Test {
    fn drop(&mut self) {
        println!("dropping!");
    }
}

impl Drop for Test2 {
    fn drop(&mut self) {}
}

fn stuff() {
    let mut x : (Test2, Test2);
    (x.0).0 = Some(Test);
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR assign of possibly uninitialized variable: `x.0`
=======
    //~^ ERROR assign of possibly-uninitialized variable: `x.0`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

fn main() {
    stuff()
}
