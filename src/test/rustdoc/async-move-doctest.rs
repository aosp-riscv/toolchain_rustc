// compile-flags:--test
// edition:2018

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// prior to setting the default edition for the doctest pre-parser, this doctest would fail due to
// a fatal parsing error
// see https://github.com/rust-lang/rust/issues/59313

//! ```
//! #![feature(async_await)]
//!
=======
// Prior to setting the default edition for the doctest pre-parser,
// this doctest would fail due to a fatal parsing error.
// see https://github.com/rust-lang/rust/issues/59313

//! ```
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
//! fn foo() {
//!     drop(async move {});
//! }
//! ```
