fn main() {
    let values: Vec<u8> = vec![1,2,3,4,5,6,7,8];

    for &[x,y,z] in values.chunks(3).filter(|&xs| xs.len() == 3) {
        //~^ ERROR refutable pattern in `for` loop binding: `&[]`, `&[_]`, `&[_, _]` and 1 more not
        println!("y={}", y);
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        //~^ ERROR borrow of possibly uninitialized variable: `y`
=======
        //~^ ERROR borrow of possibly-uninitialized variable: `y`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }
}
