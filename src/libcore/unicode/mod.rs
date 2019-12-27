#![unstable(feature = "unicode_internals", issue = "0")]
#![allow(missing_docs)]

mod bool_trie;
pub(crate) mod printable;
pub(crate) mod tables;
pub(crate) mod version;

// For use in liballoc, not re-exported in libstd.
pub mod derived_property {
    pub use crate::unicode::tables::derived_property::{Case_Ignorable, Cased};
}
pub mod conversions {
    pub use crate::unicode::tables::conversions::{to_lower, to_upper};
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
}

// For use in libsyntax
pub mod property {
    pub use crate::unicode::tables::property::Pattern_White_Space;
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
