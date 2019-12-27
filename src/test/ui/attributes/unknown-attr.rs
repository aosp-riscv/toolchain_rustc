<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// Unknown attributes fall back to feature gated custom attributes.

#![feature(custom_inner_attributes)]

#![mutable_doc]
//~^ ERROR cannot find attribute macro `mutable_doc` in this scope

#[dance] mod a {}
//~^ ERROR cannot find attribute macro `dance` in this scope

#[dance] fn main() {}
//~^ ERROR cannot find attribute macro `dance` in this scope
=======
// Unknown attributes fall back to unstable custom attributes.

#![feature(custom_inner_attributes)]

#![mutable_doc]
//~^ ERROR cannot find attribute `mutable_doc` in this scope

#[dance] mod a {}
//~^ ERROR cannot find attribute `dance` in this scope

#[dance] fn main() {}
//~^ ERROR cannot find attribute `dance` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
