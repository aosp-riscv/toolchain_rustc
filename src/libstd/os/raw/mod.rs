//! Platform-specific types, as defined by C.
//!
//! Code that interacts via FFI will almost certainly be using the
//! base types provided by C, which aren't nearly as nicely defined
//! as Rust's primitive types. This module provides types which will
//! match those defined by C, so that code that interacts with C will
//! refer to the correct types.

#![stable(feature = "raw_os", since = "1.1.0")]

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/char.md"))]
#[cfg_attr(not(bootstrap), doc(include = "char.md"))]
=======
#[doc(include = "char.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[cfg(any(all(target_os = "linux", any(target_arch = "aarch64",
                                       target_arch = "arm",
                                       target_arch = "hexagon",
                                       target_arch = "powerpc",
                                       target_arch = "powerpc64",
                                       target_arch = "s390x")),
          all(target_os = "android", any(target_arch = "aarch64",
                                         target_arch = "arm")),
          all(target_os = "l4re", target_arch = "x86_64"),
          all(target_os = "freebsd", any(target_arch = "aarch64",
                                         target_arch = "arm",
                                         target_arch = "powerpc",
                                         target_arch = "powerpc64")),
          all(target_os = "netbsd", any(target_arch = "aarch64",
                                        target_arch = "arm",
                                        target_arch = "powerpc")),
          all(target_os = "openbsd", target_arch = "aarch64"),
          all(target_os = "vxworks", any(target_arch = "aarch64",
                                         target_arch = "arm",
                                         target_arch = "powerpc64",
                                         target_arch = "powerpc")),
          all(target_os = "fuchsia", target_arch = "aarch64")))]
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_char = u8;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/char.md"))]
#[cfg_attr(not(bootstrap), doc(include = "char.md"))]
=======
#[doc(include = "char.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[cfg(not(any(all(target_os = "linux", any(target_arch = "aarch64",
                                           target_arch = "arm",
                                           target_arch = "hexagon",
                                           target_arch = "powerpc",
                                           target_arch = "powerpc64",
                                           target_arch = "s390x")),
              all(target_os = "android", any(target_arch = "aarch64",
                                             target_arch = "arm")),
              all(target_os = "l4re", target_arch = "x86_64"),
              all(target_os = "freebsd", any(target_arch = "aarch64",
                                             target_arch = "arm",
                                             target_arch = "powerpc",
                                             target_arch = "powerpc64")),
              all(target_os = "netbsd", any(target_arch = "aarch64",
                                            target_arch = "arm",
                                            target_arch = "powerpc")),
              all(target_os = "openbsd", target_arch = "aarch64"),
              all(target_os = "vxworks", any(target_arch = "aarch64",
                                             target_arch = "arm",
                                             target_arch = "powerpc64",
                                             target_arch = "powerpc")),
              all(target_os = "fuchsia", target_arch = "aarch64"))))]
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_char = i8;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/schar.md"))]
#[cfg_attr(not(bootstrap), doc(include = "schar.md"))]
=======
#[doc(include = "schar.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_schar = i8;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/uchar.md"))]
#[cfg_attr(not(bootstrap), doc(include = "uchar.md"))]
=======
#[doc(include = "uchar.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_uchar = u8;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/short.md"))]
#[cfg_attr(not(bootstrap), doc(include = "short.md"))]
=======
#[doc(include = "short.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_short = i16;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/ushort.md"))]
#[cfg_attr(not(bootstrap), doc(include = "ushort.md"))]
=======
#[doc(include = "ushort.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_ushort = u16;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/int.md"))]
#[cfg_attr(not(bootstrap), doc(include = "int.md"))]
=======
#[doc(include = "int.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_int = i32;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/uint.md"))]
#[cfg_attr(not(bootstrap), doc(include = "uint.md"))]
=======
#[doc(include = "uint.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_uint = u32;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/long.md"))]
#[cfg_attr(not(bootstrap), doc(include = "long.md"))]
=======
#[doc(include = "long.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[cfg(any(target_pointer_width = "32", windows))]
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_long = i32;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/ulong.md"))]
#[cfg_attr(not(bootstrap), doc(include = "ulong.md"))]
=======
#[doc(include = "ulong.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[cfg(any(target_pointer_width = "32", windows))]
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_ulong = u32;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/long.md"))]
#[cfg_attr(not(bootstrap), doc(include = "long.md"))]
=======
#[doc(include = "long.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[cfg(all(target_pointer_width = "64", not(windows)))]
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_long = i64;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/ulong.md"))]
#[cfg_attr(not(bootstrap), doc(include = "ulong.md"))]
=======
#[doc(include = "ulong.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[cfg(all(target_pointer_width = "64", not(windows)))]
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_ulong = u64;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/longlong.md"))]
#[cfg_attr(not(bootstrap), doc(include = "longlong.md"))]
=======
#[doc(include = "longlong.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_longlong = i64;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/ulonglong.md"))]
#[cfg_attr(not(bootstrap), doc(include = "ulonglong.md"))]
=======
#[doc(include = "ulonglong.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_ulonglong = u64;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/float.md"))]
#[cfg_attr(not(bootstrap), doc(include = "float.md"))]
=======
#[doc(include = "float.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_float = f32;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[cfg_attr(bootstrap, doc(include = "os/raw/double.md"))]
#[cfg_attr(not(bootstrap), doc(include = "double.md"))]
=======
#[doc(include = "double.md")]
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
#[stable(feature = "raw_os", since = "1.1.0")] pub type c_double = f64;

#[stable(feature = "raw_os", since = "1.1.0")]
#[doc(no_inline)]
pub use core::ffi::c_void;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::any::TypeId;
    use crate::mem;

    macro_rules! ok {
        ($($t:ident)*) => {$(
            assert!(TypeId::of::<libc::$t>() == TypeId::of::<raw::$t>(),
                    "{} is wrong", stringify!($t));
        )*}
    }

    #[test]
    fn same() {
        use crate::os::raw;
        ok!(c_char c_schar c_uchar c_short c_ushort c_int c_uint c_long c_ulong
            c_longlong c_ulonglong c_float c_double);
    }
}
