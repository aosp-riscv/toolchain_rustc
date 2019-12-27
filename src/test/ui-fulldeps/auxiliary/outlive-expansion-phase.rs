// force-host

#![feature(plugin_registrar)]
#![feature(box_syntax, rustc_private)]

extern crate rustc;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
extern crate rustc_plugin;
extern crate rustc_driver;

use std::any::Any;
use std::cell::RefCell;
use rustc_plugin::Registry;
=======
extern crate rustc_driver;

use std::any::Any;
use std::cell::RefCell;
use rustc_driver::plugin::Registry;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

struct Foo {
    foo: isize
}

impl Drop for Foo {
    fn drop(&mut self) {}
}

#[plugin_registrar]
pub fn registrar(_: &mut Registry) {
    thread_local!(static FOO: RefCell<Option<Box<Any+Send>>> = RefCell::new(None));
    FOO.with(|s| *s.borrow_mut() = Some(box Foo { foo: 10 } as Box<Any+Send>));
}
