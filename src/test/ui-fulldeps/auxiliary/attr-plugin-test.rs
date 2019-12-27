// force-host

#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate rustc_driver;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
extern crate rustc_plugin;
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
extern crate syntax;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use rustc_plugin::Registry;
=======
use rustc_driver::plugin::Registry;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
use syntax::ext::base::SyntaxExtension;
use syntax::feature_gate::AttributeType;
use syntax::symbol::Symbol;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_attribute(Symbol::intern("foo"), AttributeType::Normal);
    reg.register_attribute(Symbol::intern("bar"), AttributeType::CrateLevel);
    reg.register_attribute(Symbol::intern("baz"), AttributeType::Whitelisted);
    reg.register_syntax_extension(
        Symbol::intern("mac"), SyntaxExtension::dummy_bang(reg.sess.edition())
    );
}
