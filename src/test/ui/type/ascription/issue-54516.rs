use std::collections::BTreeMap;

fn main() {
    println!("{}", std::mem:size_of::<BTreeMap<u32, u32>>());
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR expected token: `,`
=======
    //~^ ERROR expected one of
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
