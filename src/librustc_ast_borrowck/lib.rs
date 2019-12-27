#![doc(html_root_url = "https://doc.rust-lang.org/nightly/")]

#![allow(non_camel_case_types)]

#![feature(in_band_lifetimes)]
#![feature(nll)]

#![recursion_limit="256"]

#[macro_use]
extern crate rustc;

pub use borrowck::check_crate;
pub use borrowck::build_borrowck_dataflow_data_for_fn;

mod borrowck;

pub mod graphviz;

mod dataflow;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
pub mod cfg;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

pub use borrowck::provide;
