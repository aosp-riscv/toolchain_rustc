//! Various checks
//!
//! # Note
//!
//! This API is completely unstable and subject to change.

#![doc(html_root_url = "https://doc.rust-lang.org/nightly/")]

#![feature(in_band_lifetimes)]
#![feature(nll)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(bind_by_move_pattern_guards)]
#![feature(rustc_diagnostic_macros)]
=======
#![cfg_attr(bootstrap, feature(bind_by_move_pattern_guards))]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

#![recursion_limit="256"]

#[macro_use]
extern crate rustc;

use rustc::ty::query::Providers;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
mod error_codes;
=======
pub mod error_codes;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

pub mod ast_validation;
pub mod rvalue_promotion;
pub mod hir_stats;
pub mod layout_test;
pub mod loops;

pub fn provide(providers: &mut Providers<'_>) {
    rvalue_promotion::provide(providers);
    loops::provide(providers);
}
