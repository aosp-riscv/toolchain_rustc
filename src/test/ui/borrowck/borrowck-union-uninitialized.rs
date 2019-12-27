struct S {
    a: u8,
}

union U {
    a: u8,
}

fn main() {
    unsafe {
        let mut s: S;
        let mut u: U;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        s.a = 0; //~ ERROR assign to part of possibly uninitialized variable: `s`
        u.a = 0; //~ ERROR assign to part of possibly uninitialized variable: `u`
=======
        s.a = 0; //~ ERROR assign to part of possibly-uninitialized variable: `s`
        u.a = 0; //~ ERROR assign to part of possibly-uninitialized variable: `u`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        let sa = s.a;
        let ua = u.a;
    }
}
