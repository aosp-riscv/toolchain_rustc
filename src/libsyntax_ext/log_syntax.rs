use syntax::ext::base;
use syntax::print;
use syntax::tokenstream::TokenStream;
use syntax_pos;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
pub fn expand_syntax_ext<'cx>(_cx: &'cx mut base::ExtCtxt<'_>,
=======
pub fn expand_log_syntax<'cx>(_cx: &'cx mut base::ExtCtxt<'_>,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
                              sp: syntax_pos::Span,
                              tts: TokenStream)
                              -> Box<dyn base::MacResult + 'cx> {
    println!("{}", print::pprust::tts_to_string(tts));

    // any so that `log_syntax` can be invoked as an expression and item.
    base::DummyResult::any_valid(sp)
}
