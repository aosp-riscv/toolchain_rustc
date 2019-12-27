// no-prefer-dynamic

#![crate_type = "rlib"]
#![feature(plugin_registrar, rustc_private)]

extern crate rustc;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
extern crate rustc_plugin;
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
extern crate rustc_driver;

use rustc_driver::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(_: &mut Registry) {}
