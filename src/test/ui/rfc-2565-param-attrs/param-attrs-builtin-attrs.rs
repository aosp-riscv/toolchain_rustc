<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(param_attrs)]

extern "C" {
    fn ffi(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR the attribute `test` is currently unknown to the compiler and may have
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    );
}

type FnType = fn(
    /// Foo
    //~^ ERROR documentation comments cannot be applied to function
    #[test] a: u32,
    //~^ ERROR the attribute `test` is currently unknown to the compiler and may have
    /// Bar
    //~^ ERROR documentation comments cannot be applied to function
    #[must_use]
    //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    /// Baz
    //~^ ERROR documentation comments cannot be applied to function
    #[no_mangle] b: i32,
    //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
);

pub fn foo(
    /// Foo
    //~^ ERROR documentation comments cannot be applied to function
    #[test] a: u32,
    //~^ ERROR the attribute `test` is currently unknown to the compiler and may have
    /// Bar
    //~^ ERROR documentation comments cannot be applied to function
    #[must_use]
    //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    /// Baz
    //~^ ERROR documentation comments cannot be applied to function
    #[no_mangle] b: i32,
    //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
) {}

struct SelfStruct {}
impl SelfStruct {
    fn foo(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        self,
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR the attribute `test` is currently unknown to the compiler and may have
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}
}

struct RefStruct {}
impl RefStruct {
    fn foo(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        &self,
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR the attribute `test` is currently unknown to the compiler and may have
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}
}
trait RefTrait {
    fn foo(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        &self,
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR the attribute `test` is currently unknown to the compiler and may have
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}
}
impl RefTrait for RefStruct {
    fn foo(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        &self,
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR the attribute `test` is currently unknown to the compiler and may have
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}
}

fn main() {
    let _ = |
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: u32,
        //~^ ERROR the attribute `test` is currently unknown to the compiler and may have
=======
extern "C" {
    fn ffi(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR expected an inert attribute, found an attribute macro
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    );
}

type FnType = fn(
    /// Foo
    //~^ ERROR documentation comments cannot be applied to function
    #[test] a: u32,
    //~^ ERROR expected an inert attribute, found an attribute macro
    /// Bar
    //~^ ERROR documentation comments cannot be applied to function
    #[must_use]
    //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    /// Baz
    //~^ ERROR documentation comments cannot be applied to function
    #[no_mangle] b: i32,
    //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
);

pub fn foo(
    /// Foo
    //~^ ERROR documentation comments cannot be applied to function
    #[test] a: u32,
    //~^ ERROR expected an inert attribute, found an attribute macro
    /// Bar
    //~^ ERROR documentation comments cannot be applied to function
    #[must_use]
    //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    /// Baz
    //~^ ERROR documentation comments cannot be applied to function
    #[no_mangle] b: i32,
    //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
) {}

struct SelfStruct {}
impl SelfStruct {
    fn foo(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        self,
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR expected an inert attribute, found an attribute macro
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}

    fn issue_64682_associated_fn(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR expected an inert attribute, found an attribute macro
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}
}

struct RefStruct {}
impl RefStruct {
    fn foo(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        &self,
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR expected an inert attribute, found an attribute macro
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}
}
trait RefTrait {
    fn foo(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        &self,
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR expected an inert attribute, found an attribute macro
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}

    fn issue_64682_associated_fn(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR expected an inert attribute, found an attribute macro
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}
}

impl RefTrait for RefStruct {
    fn foo(
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        &self,
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: i32,
        //~^ ERROR expected an inert attribute, found an attribute macro
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Qux
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32,
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    ) {}
}

fn main() {
    let _ = |
        /// Foo
        //~^ ERROR documentation comments cannot be applied to function
        #[test] a: u32,
        //~^ ERROR expected an inert attribute, found an attribute macro
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        /// Bar
        //~^ ERROR documentation comments cannot be applied to function
        #[must_use]
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
        /// Baz
        //~^ ERROR documentation comments cannot be applied to function
        #[no_mangle] b: i32
        //~^ ERROR allow, cfg, cfg_attr, deny, forbid, and warn are the only allowed built-in
    | {};
}
