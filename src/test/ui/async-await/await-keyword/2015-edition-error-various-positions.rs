<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(async_await)]
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#![allow(non_camel_case_types)]
#![deny(keyword_idents)]

mod outer_mod {
    pub mod await { //~ ERROR `await` is a keyword in the 2018 edition
    //~^ WARN this was previously accepted by the compiler
        pub struct await; //~ ERROR `await` is a keyword in the 2018 edition
        //~^ WARN this was previously accepted by the compiler
    }
}
use outer_mod::await::await; //~ ERROR `await` is a keyword in the 2018 edition
//~^ ERROR `await` is a keyword in the 2018 edition
//~^^ WARN this was previously accepted by the compiler
//~^^^ WARN this was previously accepted by the compiler

struct Foo { await: () }
//~^ ERROR `await` is a keyword in the 2018 edition
//~^^ WARN this was previously accepted by the compiler

impl Foo { fn await() {} }
//~^ ERROR `await` is a keyword in the 2018 edition
//~^^ WARN this was previously accepted by the compiler

macro_rules! await {
//~^ ERROR `await` is a keyword in the 2018 edition
//~^^ WARN this was previously accepted by the compiler
    () => {}
}

fn main() {
    await!(); //~ ERROR `await` is a keyword in the 2018 edition
    //~^ WARN this was previously accepted by the compiler

    match await { await => {} } //~ ERROR `await` is a keyword in the 2018 edition
    //~^ ERROR `await` is a keyword in the 2018 edition
    //~^^ WARN this was previously accepted by the compiler
    //~^^^ WARN this was previously accepted by the compiler
}
