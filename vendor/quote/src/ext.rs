use super::ToTokens;

use std::iter;

use proc_macro2::{TokenStream, TokenTree};

/// TokenStream extension trait with methods for appending tokens.
///
/// This trait is sealed and cannot be implemented outside of the `quote` crate.
pub trait TokenStreamExt: private::Sealed {
    /// For use by `ToTokens` implementations.
    ///
    /// Appends the token specified to this list of tokens.
    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>;

    /// For use by `ToTokens` implementations.
    ///
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    /// ```edition2018
=======
    /// ```
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    /// # use quote::{quote, TokenStreamExt, ToTokens};
    /// # use proc_macro2::TokenStream;
    /// #
    /// struct X;
    ///
    /// impl ToTokens for X {
    ///     fn to_tokens(&self, tokens: &mut TokenStream) {
    ///         tokens.append_all(&[true, false]);
    ///     }
    /// }
    ///
    /// let tokens = quote!(#X);
    /// assert_eq!(tokens.to_string(), "true false");
    /// ```
    fn append_all<I>(&mut self, iter: I)
    where
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        T: ToTokens,
        I: IntoIterator<Item = T>;

    /// For use by `ToTokens` implementations.
    ///
    /// Appends all of the items in the iterator `I`, separated by the tokens
    /// `U`.
    fn append_separated<T, I, U>(&mut self, iter: I, op: U)
    where
        T: ToTokens,
        I: IntoIterator<Item = T>,
        U: ToTokens;

    /// For use by `ToTokens` implementations.
    ///
    /// Appends all tokens in the iterator `I`, appending `U` after each
    /// element, including after the last element of the iterator.
    fn append_terminated<T, I, U>(&mut self, iter: I, term: U)
    where
        T: ToTokens,
        I: IntoIterator<Item = T>,
        U: ToTokens;
}

impl TokenStreamExt for TokenStream {
    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.extend(iter::once(token.into()));
    }

    fn append_all<T, I>(&mut self, iter: I)
    where
        T: ToTokens,
        I: IntoIterator<Item = T>,
=======
        I: IntoIterator,
        I::Item: ToTokens;

    /// For use by `ToTokens` implementations.
    ///
    /// Appends all of the items in the iterator `I`, separated by the tokens
    /// `U`.
    fn append_separated<I, U>(&mut self, iter: I, op: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens;

    /// For use by `ToTokens` implementations.
    ///
    /// Appends all tokens in the iterator `I`, appending `U` after each
    /// element, including after the last element of the iterator.
    fn append_terminated<I, U>(&mut self, iter: I, term: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens;
}

impl TokenStreamExt for TokenStream {
    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.extend(iter::once(token.into()));
    }

    fn append_all<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToTokens,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    {
        for token in iter {
            token.to_tokens(self);
        }
    }

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    fn append_separated<T, I, U>(&mut self, iter: I, op: U)
=======
    fn append_separated<I, U>(&mut self, iter: I, op: U)
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens,
    {
        for (i, token) in iter.into_iter().enumerate() {
            if i > 0 {
                op.to_tokens(self);
            }
            token.to_tokens(self);
        }
    }

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    fn append_terminated<T, I, U>(&mut self, iter: I, term: U)
=======
    fn append_terminated<I, U>(&mut self, iter: I, term: U)
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens,
    {
        for token in iter {
            token.to_tokens(self);
            term.to_tokens(self);
        }
    }
}

mod private {
    use proc_macro2::TokenStream;

    pub trait Sealed {}

    impl Sealed for TokenStream {}
}
