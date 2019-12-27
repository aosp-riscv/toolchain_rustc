//! Extension traits to provide parsing methods on foreign types.
//!
//! *This module is available if Syn is built with the `"parsing"` feature.*

use proc_macro2::Ident;

use crate::parse::{ParseStream, Result};

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg(syn_can_use_associated_constants)]
use buffer::Cursor;
#[cfg(syn_can_use_associated_constants)]
use parse::Peek;
#[cfg(syn_can_use_associated_constants)]
use sealed::lookahead;
#[cfg(syn_can_use_associated_constants)]
use token::CustomToken;
=======
use crate::buffer::Cursor;
use crate::parse::Peek;
use crate::sealed::lookahead;
use crate::token::CustomToken;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

/// Additional methods for `Ident` not provided by proc-macro2 or libproc_macro.
///
/// This trait is sealed and cannot be implemented for types outside of Syn. It
/// is implemented only for `proc_macro2::Ident`.
///
/// *This trait is available if Syn is built with the `"parsing"` feature.*
pub trait IdentExt: Sized + private::Sealed {
    /// Parses any identifier including keywords.
    ///
    /// This is useful when parsing macro input which allows Rust keywords as
    /// identifiers.
    ///
    /// # Example
    ///
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    /// ```edition2018
=======
    /// ```
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    /// use syn::{Error, Ident, Result, Token};
    /// use syn::ext::IdentExt;
    /// use syn::parse::ParseStream;
    ///
    /// mod kw {
    ///     syn::custom_keyword!(name);
    /// }
    ///
    /// // Parses input that looks like `name = NAME` where `NAME` can be
    /// // any identifier.
    /// //
    /// // Examples:
    /// //
    /// //     name = anything
    /// //     name = impl
    /// fn parse_dsl(input: ParseStream) -> Result<Ident> {
    ///     input.parse::<kw::name>()?;
    ///     input.parse::<Token![=]>()?;
    ///     let name = input.call(Ident::parse_any)?;
    ///     Ok(name)
    /// }
    /// ```
    fn parse_any(input: ParseStream) -> Result<Self>;

    /// Peeks any identifier including keywords. Usage:
    /// `input.peek(Ident::peek_any)`
    ///
    /// This is different from `input.peek(Ident)` which only returns true in
    /// the case of an ident which is not a Rust keyword.
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    #[cfg(syn_can_use_associated_constants)]
    #[allow(non_upper_case_globals)]
    const peek_any: private::PeekFn = private::PeekFn;

    /// Strips the raw marker `r#`, if any, from the beginning of an ident.
    ///
    ///   - unraw(`x`) = `x`
    ///   - unraw(`move`) = `move`
    ///   - unraw(`r#move`) = `move`
    ///
    /// # Example
    ///
    /// In the case of interop with other languages like Python that have a
    /// different set of keywords than Rust, we might come across macro input
    /// that involves raw identifiers to refer to ordinary variables in the
    /// other language with a name that happens to be a Rust keyword.
    ///
    /// The function below appends an identifier from the caller's input onto a
    /// fixed prefix. Without using `unraw()`, this would tend to produce
    /// invalid identifiers like `__pyo3_get_r#move`.
    ///
    /// ```edition2018
=======
    #[allow(non_upper_case_globals)]
    const peek_any: private::PeekFn = private::PeekFn;

    /// Strips the raw marker `r#`, if any, from the beginning of an ident.
    ///
    ///   - unraw(`x`) = `x`
    ///   - unraw(`move`) = `move`
    ///   - unraw(`r#move`) = `move`
    ///
    /// # Example
    ///
    /// In the case of interop with other languages like Python that have a
    /// different set of keywords than Rust, we might come across macro input
    /// that involves raw identifiers to refer to ordinary variables in the
    /// other language with a name that happens to be a Rust keyword.
    ///
    /// The function below appends an identifier from the caller's input onto a
    /// fixed prefix. Without using `unraw()`, this would tend to produce
    /// invalid identifiers like `__pyo3_get_r#move`.
    ///
    /// ```
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    /// use proc_macro2::Span;
    /// use syn::Ident;
    /// use syn::ext::IdentExt;
    ///
    /// fn ident_for_getter(variable: &Ident) -> Ident {
    ///     let getter = format!("__pyo3_get_{}", variable.unraw());
    ///     Ident::new(&getter, Span::call_site())
    /// }
    /// ```
    fn unraw(&self) -> Ident;
}

impl IdentExt for Ident {
    fn parse_any(input: ParseStream) -> Result<Self> {
        input.step(|cursor| match cursor.ident() {
            Some((ident, rest)) => Ok((ident, rest)),
            None => Err(cursor.error("expected ident")),
        })
    }

    fn unraw(&self) -> Ident {
        let string = self.to_string();
        if string.starts_with("r#") {
            Ident::new(&string[2..], self.span())
        } else {
            self.clone()
        }
    }
}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg(syn_can_use_associated_constants)]
impl Peek for private::PeekFn {
    type Token = private::IdentAny;
}

#[cfg(syn_can_use_associated_constants)]
impl CustomToken for private::IdentAny {
    fn peek(cursor: Cursor) -> bool {
        cursor.ident().is_some()
    }

    fn display() -> &'static str {
        "identifier"
    }
}

#[cfg(syn_can_use_associated_constants)]
=======
impl Peek for private::PeekFn {
    type Token = private::IdentAny;
}

impl CustomToken for private::IdentAny {
    fn peek(cursor: Cursor) -> bool {
        cursor.ident().is_some()
    }

    fn display() -> &'static str {
        "identifier"
    }
}

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
impl lookahead::Sealed for private::PeekFn {}

mod private {
    use proc_macro2::Ident;

    pub trait Sealed {}

    impl Sealed for Ident {}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    #[cfg(syn_can_use_associated_constants)]
    #[derive(Copy, Clone)]
    pub struct PeekFn;
    #[cfg(syn_can_use_associated_constants)]
=======
    #[derive(Copy, Clone)]
    pub struct PeekFn;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    pub struct IdentAny;
}
