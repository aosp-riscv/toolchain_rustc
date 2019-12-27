//! Platform dependent types.

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::borrow::Cow;
        use std::fmt;
        use std::path::PathBuf;
        use std::prelude::v1::*;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
        use std::str;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }
}

/// A platform independent representation of a string. When working with `std`
/// enabled it is recommended to the convenience methods for providing
/// conversions to `std` types.
#[derive(Debug)]
pub enum BytesOrWideString<'a> {
    /// A slice, typically provided on Unix platforms.
    Bytes(&'a [u8]),
    /// Wide strings typically from Windows.
    Wide(&'a [u16]),
}

#[cfg(feature = "std")]
impl<'a> BytesOrWideString<'a> {
    /// Lossy converts to a `Cow<str>`, will allocate if `Bytes` is not valid
    /// UTF-8 or if `BytesOrWideString` is `Wide`.
    ///
    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn to_str_lossy(&self) -> Cow<'a, str> {
        use self::BytesOrWideString::*;

        match self {
            &Bytes(slice) => String::from_utf8_lossy(slice),
            &Wide(wide) => Cow::Owned(String::from_utf16_lossy(wide)),
        }
    }

    /// Provides a `Path` representation of `BytesOrWideString`.
    ///
    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    pub fn into_path_buf(self) -> PathBuf {
        #[cfg(unix)]
        {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
            use self::BytesOrWideString::*;
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
            match self {
                Bytes(slice) => PathBuf::from(OsStr::from_bytes(slice)),
                _ => unreachable!(),
=======
            if let BytesOrWideString::Bytes(slice) = self {
                return PathBuf::from(OsStr::from_bytes(slice));
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
            }
        }

        #[cfg(windows)]
        {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
            use self::BytesOrWideString::*;
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
            match self {
                Wide(slice) => PathBuf::from(OsString::from_wide(slice)),
                _ => unreachable!(),
            }
        }

        #[cfg(all(not(windows), not(unix)))]
        {
            unreachable!()
=======
            if let BytesOrWideString::Wide(slice) = self {
                return PathBuf::from(OsString::from_wide(slice));
            }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        }

        if let BytesOrWideString::Bytes(b) = self {
            if let Ok(s) = str::from_utf8(b) {
                return PathBuf::from(s);
            }
        }
        unreachable!()
    }
}

#[cfg(feature = "std")]
impl<'a> fmt::Display for BytesOrWideString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_str_lossy().fmt(f)
    }
}
