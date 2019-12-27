#![feature(never_type)]

enum Helper<T, U> {
    T(T, [!; 0]),
    #[allow(dead_code)]
    U(U),
}

fn transmute<T, U>(t: T) -> U {
    let Helper::U(u) = Helper::T(t, []);
    //~^ ERROR refutable pattern in local binding: `T(_, _)` not covered
    u
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR use of possibly uninitialized variable: `u`
=======
    //~^ ERROR use of possibly-uninitialized variable: `u`
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

fn main() {
    println!("{:?}", transmute::<&str, (*const u8, u64)>("type safety"));
}
