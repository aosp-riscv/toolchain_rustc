fn main() {}

const fn slice([a, b]: &[i32]) -> i32 { //~ ERROR refutable pattern in function argument
    a + b //~ ERROR can only call other `const fn` within a `const fn`
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR use of possibly uninitialized variable: `a`
    //~| ERROR use of possibly uninitialized variable: `b`
=======
    //~^ ERROR use of possibly-uninitialized variable: `a`
    //~| ERROR use of possibly-uninitialized variable: `b`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
