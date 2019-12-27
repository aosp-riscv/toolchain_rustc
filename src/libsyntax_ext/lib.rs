//! This crate contains implementations of built-in macros and other code generating facilities
//! injecting code into the crate before it is lowered to HIR.

#![doc(html_root_url = "https://doc.rust-lang.org/nightly/")]

#![feature(crate_visibility_modifier)]
#![feature(decl_macro)]
#![feature(mem_take)]
#![feature(nll)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#![feature(rustc_diagnostic_macros)]

use crate::deriving::*;
=======
#![feature(proc_macro_internals)]
#![feature(proc_macro_quote)]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

use syntax::ast::Ident;
use syntax::edition::Edition;
use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind, MacroExpanderFn};
use syntax::symbol::sym;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
use crate::deriving::*;

use syntax::ast::Ident;
use syntax::edition::Edition;
use syntax::ext::base::{SyntaxExtension, SyntaxExtensionKind, MacroExpanderFn};
use syntax::ext::proc_macro::BangProcMacro;
use syntax::symbol::sym;

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
mod error_codes;

mod asm;
mod assert;
mod cfg;
mod compile_error;
mod concat;
mod concat_idents;
mod deriving;
mod env;
mod format;
mod format_foreign;
mod global_allocator;
mod global_asm;
mod log_syntax;
mod source_util;
mod test;
mod trace_macros;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
pub mod cmdline_attrs;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
pub mod plugin_macro_defs;
pub mod proc_macro_harness;
pub mod standard_library_imports;
pub mod test_harness;

pub fn register_builtin_macros(resolver: &mut dyn syntax::ext::base::Resolver, edition: Edition) {
    let mut register = |name, kind| resolver.register_builtin_macro(
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        Ident::with_empty_ctxt(name), SyntaxExtension {
=======
        Ident::with_dummy_span(name), SyntaxExtension {
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
            is_builtin: true, ..SyntaxExtension::default(kind, edition)
        },
    );
    macro register_bang($($name:ident: $f:expr,)*) {
        $(register(sym::$name, SyntaxExtensionKind::LegacyBang(Box::new($f as MacroExpanderFn)));)*
    }
    macro register_attr($($name:ident: $f:expr,)*) {
        $(register(sym::$name, SyntaxExtensionKind::LegacyAttr(Box::new($f)));)*
    }
    macro register_derive($($name:ident: $f:expr,)*) {
        $(register(sym::$name, SyntaxExtensionKind::LegacyDerive(Box::new(BuiltinDerive($f))));)*
    }

    register_bang! {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        __rust_unstable_column: source_util::expand_column,
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        asm: asm::expand_asm,
        assert: assert::expand_assert,
        cfg: cfg::expand_cfg,
        column: source_util::expand_column,
        compile_error: compile_error::expand_compile_error,
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        concat_idents: concat_idents::expand_syntax_ext,
        concat: concat::expand_syntax_ext,
        env: env::expand_env,
        file: source_util::expand_file,
        format_args_nl: format::expand_format_args_nl,
        format_args: format::expand_format_args,
        global_asm: global_asm::expand_global_asm,
        include_bytes: source_util::expand_include_bytes,
        include_str: source_util::expand_include_str,
        include: source_util::expand_include,
        line: source_util::expand_line,
        log_syntax: log_syntax::expand_syntax_ext,
=======
        concat_idents: concat_idents::expand_concat_idents,
        concat: concat::expand_concat,
        env: env::expand_env,
        file: source_util::expand_file,
        format_args_nl: format::expand_format_args_nl,
        format_args: format::expand_format_args,
        global_asm: global_asm::expand_global_asm,
        include_bytes: source_util::expand_include_bytes,
        include_str: source_util::expand_include_str,
        include: source_util::expand_include,
        line: source_util::expand_line,
        log_syntax: log_syntax::expand_log_syntax,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        module_path: source_util::expand_mod,
        option_env: env::expand_option_env,
        stringify: source_util::expand_stringify,
        trace_macros: trace_macros::expand_trace_macros,
    }

    register_attr! {
        bench: test::expand_bench,
        global_allocator: global_allocator::expand,
        test: test::expand_test,
        test_case: test::expand_test_case,
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    }

    register_derive! {
        Clone: clone::expand_deriving_clone,
        Copy: bounds::expand_deriving_copy,
        Debug: debug::expand_deriving_debug,
        Default: default::expand_deriving_default,
        Eq: eq::expand_deriving_eq,
        Hash: hash::expand_deriving_hash,
        Ord: ord::expand_deriving_ord,
        PartialEq: partial_eq::expand_deriving_partial_eq,
        PartialOrd: partial_ord::expand_deriving_partial_ord,
        RustcDecodable: decodable::expand_deriving_rustc_decodable,
        RustcEncodable: encodable::expand_deriving_rustc_encodable,
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }

    register_derive! {
        Clone: clone::expand_deriving_clone,
        Copy: bounds::expand_deriving_copy,
        Debug: debug::expand_deriving_debug,
        Default: default::expand_deriving_default,
        Eq: eq::expand_deriving_eq,
        Hash: hash::expand_deriving_hash,
        Ord: ord::expand_deriving_ord,
        PartialEq: partial_eq::expand_deriving_partial_eq,
        PartialOrd: partial_ord::expand_deriving_partial_ord,
        RustcDecodable: decodable::expand_deriving_rustc_decodable,
        RustcEncodable: encodable::expand_deriving_rustc_encodable,
    }

    let client = proc_macro::bridge::client::Client::expand1(proc_macro::quote);
    register(sym::quote, SyntaxExtensionKind::Bang(Box::new(BangProcMacro { client })));
}
