use syntax::{ast, attr};
use syntax::edition::Edition;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use syntax::ext::hygiene::{ExpnId, MacroKind};
use syntax::ptr::P;
use syntax::source_map::{ExpnInfo, ExpnKind, dummy_spanned, respan};
use syntax::symbol::{Ident, Symbol, kw, sym};
use syntax_pos::DUMMY_SP;

use std::iter;

pub fn inject(
    mut krate: ast::Crate, alt_std_name: Option<&str>, edition: Edition
) -> (ast::Crate, Option<Symbol>) {
    let rust_2018 = edition >= Edition::Edition2018;

    // the first name in this list is the crate name of the crate with the prelude
    let names: &[&str] = if attr::contains_name(&krate.attrs, sym::no_core) {
        return (krate, None);
    } else if attr::contains_name(&krate.attrs, sym::no_std) {
        if attr::contains_name(&krate.attrs, sym::compiler_builtins) {
            &["core"]
        } else {
            &["core", "compiler_builtins"]
        }
    } else {
        &["std"]
    };

    // .rev() to preserve ordering above in combination with insert(0, ...)
    let alt_std_name = alt_std_name.map(Symbol::intern);
    for orig_name_str in names.iter().rev() {
        // HACK(eddyb) gensym the injected crates on the Rust 2018 edition,
        // so they don't accidentally interfere with the new import paths.
        let orig_name_sym = Symbol::intern(orig_name_str);
        let orig_name_ident = Ident::with_empty_ctxt(orig_name_sym);
        let (rename, orig_name) = if rust_2018 {
            (orig_name_ident.gensym(), Some(orig_name_sym))
        } else {
            (orig_name_ident, None)
        };
        krate.module.items.insert(0, P(ast::Item {
            attrs: vec![attr::mk_attr_outer(
                attr::mk_word_item(ast::Ident::with_empty_ctxt(sym::macro_use))
            )],
            vis: dummy_spanned(ast::VisibilityKind::Inherited),
            node: ast::ItemKind::ExternCrate(alt_std_name.or(orig_name)),
            ident: rename,
            id: ast::DUMMY_NODE_ID,
            span: DUMMY_SP,
            tokens: None,
        }));
    }

    // the crates have been injected, the assumption is that the first one is the one with
    // the prelude.
    let name = names[0];

    let span = DUMMY_SP.fresh_expansion(ExpnId::root(), ExpnInfo::allow_unstable(
        ExpnKind::Macro(MacroKind::Attr, sym::std_inject), DUMMY_SP, edition,
        [sym::prelude_import][..].into(),
    ));

    krate.module.items.insert(0, P(ast::Item {
        attrs: vec![attr::mk_attr_outer(
            attr::mk_word_item(ast::Ident::new(sym::prelude_import, span)))],
        vis: respan(span.shrink_to_lo(), ast::VisibilityKind::Inherited),
        node: ast::ItemKind::Use(P(ast::UseTree {
            prefix: ast::Path {
                segments: iter::once(ast::Ident::with_empty_ctxt(kw::PathRoot))
                    .chain(
                        [name, "prelude", "v1"].iter().cloned()
                            .map(ast::Ident::from_str)
                    ).map(ast::PathSegment::from_ident).collect(),
                span,
            },
            kind: ast::UseTreeKind::Glob,
            span,
        })),
        id: ast::DUMMY_NODE_ID,
        ident: ast::Ident::invalid(),
        span,
        tokens: None,
    }));

    (krate, Some(Symbol::intern(name)))
=======
use syntax::ext::expand::ExpansionConfig;
use syntax::ext::hygiene::AstPass;
use syntax::ext::base::{ExtCtxt, Resolver};
use syntax::parse::ParseSess;
use syntax::ptr::P;
use syntax::symbol::{Ident, Symbol, kw, sym};
use syntax_pos::DUMMY_SP;

pub fn inject(
    mut krate: ast::Crate,
    resolver: &mut dyn Resolver,
    sess: &ParseSess,
    alt_std_name: Option<Symbol>,
) -> (ast::Crate, Option<Symbol>) {
    let rust_2018 = sess.edition >= Edition::Edition2018;

    // the first name in this list is the crate name of the crate with the prelude
    let names: &[Symbol] = if attr::contains_name(&krate.attrs, sym::no_core) {
        return (krate, None);
    } else if attr::contains_name(&krate.attrs, sym::no_std) {
        if attr::contains_name(&krate.attrs, sym::compiler_builtins) {
            &[sym::core]
        } else {
            &[sym::core, sym::compiler_builtins]
        }
    } else {
        &[sym::std]
    };

    let expn_id = resolver.expansion_for_ast_pass(
        DUMMY_SP,
        AstPass::StdImports,
        &[sym::prelude_import],
        None,
    );
    let span = DUMMY_SP.with_def_site_ctxt(expn_id);
    let call_site = DUMMY_SP.with_call_site_ctxt(expn_id);

    let ecfg = ExpansionConfig::default("std_lib_injection".to_string());
    let cx = ExtCtxt::new(sess, ecfg, resolver);


    // .rev() to preserve ordering above in combination with insert(0, ...)
    for &name in names.iter().rev() {
        let ident = if rust_2018 {
            Ident::new(name, span)
        } else {
            Ident::new(name, call_site)
        };
        krate.module.items.insert(0, cx.item(
            span,
            ident,
            vec![cx.attribute(cx.meta_word(span, sym::macro_use))],
            ast::ItemKind::ExternCrate(alt_std_name),
        ));
    }

    // The crates have been injected, the assumption is that the first one is
    // the one with the prelude.
    let name = names[0];

    let import_path = if rust_2018 {
        [name, sym::prelude, sym::v1].iter()
            .map(|symbol| ast::Ident::new(*symbol, span)).collect()
    } else {
        [kw::PathRoot, name, sym::prelude, sym::v1].iter()
            .map(|symbol| ast::Ident::new(*symbol, span)).collect()
    };

    let use_item = cx.item(
        span,
        ast::Ident::invalid(),
        vec![cx.attribute(cx.meta_word(span, sym::prelude_import))],
        ast::ItemKind::Use(P(ast::UseTree {
            prefix: cx.path(span, import_path),
            kind: ast::UseTreeKind::Glob,
            span,
        })),
    );

    krate.module.items.insert(0, use_item);

    (krate, Some(name))
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
