fn main() {
    let x = -5;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    if x<-1 { //~ ERROR expected `{`, found `<-`
=======
    if x<-1 { //~ ERROR unexpected token: `<-`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        println!("ok");
    }
}
