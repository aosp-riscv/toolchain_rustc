#![feature(proc_macro_hygiene)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![cfg_attr(not(bootstrap), allow(rustc::default_hash_types))]
=======
#![allow(rustc::default_hash_types)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

#![recursion_limit="128"]

extern crate proc_macro;

use synstructure::decl_derive;

use proc_macro::TokenStream;

mod hash_stable;
mod query;
mod symbols;

#[proc_macro]
pub fn rustc_queries(input: TokenStream) -> TokenStream {
    query::rustc_queries(input)
}

#[proc_macro]
pub fn symbols(input: TokenStream) -> TokenStream {
    symbols::symbols(input)
}

decl_derive!([HashStable, attributes(stable_hasher)] => hash_stable::hash_stable_derive);
