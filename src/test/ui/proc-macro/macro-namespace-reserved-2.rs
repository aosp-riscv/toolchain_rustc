// force-host
// no-prefer-dynamic

#![crate_type = "proc-macro"]

extern crate proc_macro;
use proc_macro::*;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn my_macro_attr(input: TokenStream, _: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(MyTrait)]
pub fn my_macro_derive(input: TokenStream) -> TokenStream {
    input
}

fn check_bang1() {
    my_macro!(); //~ ERROR can't use a procedural macro from the same crate that defines it
}
fn check_bang2() {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    my_macro_attr!(); //~ ERROR cannot find macro `my_macro_attr!` in this scope
=======
    my_macro_attr!(); //~ ERROR cannot find macro `my_macro_attr` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    crate::my_macro_attr!(); //~ ERROR can't use a procedural macro from the same crate that defines
                             //~| ERROR expected macro, found attribute macro `crate::my_macro_attr`
}
fn check_bang3() {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    MyTrait!(); //~ ERROR cannot find macro `MyTrait!` in this scope
=======
    MyTrait!(); //~ ERROR cannot find macro `MyTrait` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    crate::MyTrait!(); //~ ERROR can't use a procedural macro from the same crate that defines it
                       //~| ERROR expected macro, found derive macro `crate::MyTrait`
}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[my_macro] //~ ERROR cannot find attribute macro `my_macro` in this scope
=======
#[my_macro] //~ ERROR cannot find attribute `my_macro` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[crate::my_macro] //~ ERROR can't use a procedural macro from the same crate that defines it
                   //~| ERROR expected attribute, found macro `crate::my_macro`
fn check_attr1() {}
#[my_macro_attr] //~ ERROR can't use a procedural macro from the same crate that defines it
fn check_attr2() {}
#[MyTrait] //~ ERROR can't use a procedural macro from the same crate that defines it
           //~| ERROR expected attribute, found derive macro `MyTrait`
fn check_attr3() {}

#[derive(my_macro)] //~ ERROR cannot find derive macro `my_macro` in this scope
#[derive(crate::my_macro)] //~ ERROR can't use a procedural macro from the same crate that defines
                           //~| ERROR expected derive macro, found macro `crate::my_macro`
struct CheckDerive1;
#[derive(my_macro_attr)] //~ ERROR can't use a procedural macro from the same crate that defines it
                         //~| ERROR expected derive macro, found attribute macro `my_macro_attr`
struct CheckDerive2;
#[derive(MyTrait)] //~ ERROR can't use a procedural macro from the same crate that defines it
struct CheckDerive3;
