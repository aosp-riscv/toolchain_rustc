extern crate proc_macro;

use proc_macro::*;

#[proc_macro_attribute]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
pub fn cargo_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let mut ret = TokenStream::new();
    ret.extend(Some(TokenTree::from(Punct::new('#', Spacing::Alone))));
    let test = TokenTree::from(Ident::new("test", span));
    ret.extend(Some(TokenTree::from(Group::new(
        Delimiter::Bracket,
        test.into(),
    ))));

    for token in item {
        let group = match token {
            TokenTree::Group(g) => {
                if g.delimiter() == Delimiter::Brace {
                    g
                } else {
                    ret.extend(Some(TokenTree::Group(g)));
                    continue;
                }
            }
            other => {
                ret.extend(Some(other));
                continue;
            }
        };

        let mut new_body = vec![
            TokenTree::from(Ident::new("let", span)),
            TokenTree::from(Ident::new("_test_guard", span)),
            TokenTree::from(Punct::new('=', Spacing::Alone)),
            TokenTree::from(Ident::new("crate", span)),
            TokenTree::from(Punct::new(':', Spacing::Joint)),
            TokenTree::from(Punct::new(':', Spacing::Alone)),
            TokenTree::from(Ident::new("support", span)),
            TokenTree::from(Punct::new(':', Spacing::Joint)),
            TokenTree::from(Punct::new(':', Spacing::Alone)),
            TokenTree::from(Ident::new("paths", span)),
            TokenTree::from(Punct::new(':', Spacing::Joint)),
            TokenTree::from(Punct::new(':', Spacing::Alone)),
            TokenTree::from(Ident::new("init_root", span)),
            TokenTree::from(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::from(Punct::new(';', Spacing::Alone)),
        ]
        .into_iter()
        .collect::<TokenStream>();
        new_body.extend(group.stream());
        ret.extend(Some(TokenTree::from(Group::new(
            group.delimiter(),
            new_body,
        ))));
    }

    return ret;
=======
pub fn cargo_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let mut ret = TokenStream::new();
    ret.extend(Some(TokenTree::from(Punct::new('#', Spacing::Alone))));
    let test = TokenTree::from(Ident::new("test", span));
    ret.extend(Some(TokenTree::from(Group::new(
        Delimiter::Bracket,
        test.into(),
    ))));

    let build_std = contains_ident(&attr, "build_std");

    for token in item {
        let group = match token {
            TokenTree::Group(g) => {
                if g.delimiter() == Delimiter::Brace {
                    g
                } else {
                    ret.extend(Some(TokenTree::Group(g)));
                    continue;
                }
            }
            other => {
                ret.extend(Some(other));
                continue;
            }
        };

        let mut new_body =
            to_token_stream("let _test_guard = cargo_test_support::paths::init_root();");

        // If this is a `build_std` test (aka `tests/build-std/*.rs`) then they
        // only run on nightly and they only run when specifically instructed to
        // on CI.
        if build_std {
            let ts = to_token_stream("if !cargo_test_support::is_nightly() { return }");
            new_body.extend(ts);
            let ts = to_token_stream(
                "if std::env::var(\"CARGO_RUN_BUILD_STD_TESTS\").is_err() { return }",
            );
            new_body.extend(ts);
        }
        new_body.extend(group.stream());
        ret.extend(Some(TokenTree::from(Group::new(
            group.delimiter(),
            new_body,
        ))));
    }

    return ret;
}

fn contains_ident(t: &TokenStream, ident: &str) -> bool {
    t.clone().into_iter().any(|t| match t {
        TokenTree::Ident(i) => i.to_string() == ident,
        _ => false,
    })
}

fn to_token_stream(code: &str) -> TokenStream {
    code.parse().unwrap()
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
