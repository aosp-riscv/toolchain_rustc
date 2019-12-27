/// The compiler code necessary to support the cfg! extension, which expands to
/// a literal `true` or `false` based on whether the given cfg matches the
/// current compilation environment.

use errors::DiagnosticBuilder;

use syntax::ast;
use syntax::ext::base::{self, *};
use syntax::attr;
use syntax::tokenstream::TokenStream;
use syntax::parse::token;
use syntax_pos::Span;

pub fn expand_cfg(
    cx: &mut ExtCtxt<'_>,
    sp: Span,
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    tts: &[tokenstream::TokenTree],
) -> Box<dyn base::MacResult + 'static> {
    let sp = sp.apply_mark(cx.current_expansion.id);
=======
    tts: TokenStream,
) -> Box<dyn base::MacResult + 'static> {
    let sp = cx.with_def_site_ctxt(sp);
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

    match parse_cfg(cx, sp, tts) {
        Ok(cfg) => {
            let matches_cfg = attr::cfg_matches(&cfg, cx.parse_sess, cx.ecfg.features);
            MacEager::expr(cx.expr_bool(sp, matches_cfg))
        }
        Err(mut err) => {
            err.emit();
            DummyResult::any(sp)
        }
    }
}

fn parse_cfg<'a>(
    cx: &mut ExtCtxt<'a>,
    sp: Span,
    tts: TokenStream,
) -> Result<ast::MetaItem, DiagnosticBuilder<'a>> {
    let mut p = cx.new_parser_from_tts(tts);

    if p.token == token::Eof {
        let mut err = cx.struct_span_err(sp, "macro requires a cfg-pattern as an argument");
        err.span_label(sp, "cfg-pattern required");
        return Err(err);
    }

    let cfg = p.parse_meta_item()?;

    let _ = p.eat(&token::Comma);

    if !p.eat(&token::Eof) {
        return Err(cx.struct_span_err(sp, "expected 1 cfg-pattern"));
    }

    Ok(cfg)
}
