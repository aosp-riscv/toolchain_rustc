// force-host

#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate rustc;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
extern crate rustc_plugin;
extern crate rustc_driver;

use rustc_plugin::Registry;
=======
extern crate rustc_driver;

use rustc_driver::plugin::Registry;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

#[plugin_registrar]
pub fn plugin_registrar(_reg: &mut Registry) {}
