use syntax::ext::base::{self, ExtCtxt};
use syntax::symbol::kw;
use syntax_pos::Span;
use syntax::tokenstream::{TokenTree, TokenStream};

pub fn expand_trace_macros(cx: &mut ExtCtxt<'_>,
                           sp: Span,
                           tt: TokenStream)
                           -> Box<dyn base::MacResult + 'static> {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    match tt {
        [TokenTree::Token(token)] if token.is_keyword(kw::True) => {
            cx.set_trace_macros(true);
        }
        [TokenTree::Token(token)] if token.is_keyword(kw::False) => {
            cx.set_trace_macros(false);
        }
        _ => cx.span_err(sp, "trace_macros! accepts only `true` or `false`"),
=======
    let mut cursor = tt.into_trees();
    let mut err = false;
    let value = match &cursor.next() {
        Some(TokenTree::Token(token)) if token.is_keyword(kw::True) => true,
        Some(TokenTree::Token(token)) if token.is_keyword(kw::False) => false,
        _ => {
            err = true;
            false
        },
    };
    err |= cursor.next().is_some();
    if err {
        cx.span_err(sp, "trace_macros! accepts only `true` or `false`")
    } else {
        cx.set_trace_macros(value);
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }

    base::DummyResult::any_valid(sp)
}
