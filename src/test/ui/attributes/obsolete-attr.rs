<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
// Obsolete attributes fall back to feature gated custom attributes.

#[ab_isize="stdcall"] extern {}
//~^ ERROR cannot find attribute macro `ab_isize` in this scope

#[fixed_stack_segment] fn f() {}
//~^ ERROR cannot find attribute macro `fixed_stack_segment` in this scope
=======
// Obsolete attributes fall back to unstable custom attributes.

#[ab_isize="stdcall"] extern {}
//~^ ERROR cannot find attribute `ab_isize` in this scope

#[fixed_stack_segment] fn f() {}
//~^ ERROR cannot find attribute `fixed_stack_segment` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

fn main() {}
