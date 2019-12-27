#![deny(clippy::temporary_cstring_as_ptr)]

fn main() {}

fn temporary_cstring() {
    use std::ffi::CString;

    CString::new("foo").unwrap().as_ptr();
    CString::new("foo").expect("dummy").as_ptr();
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
}

mod issue4375 {
    use std::ffi::CString;
    use std::os::raw::c_char;

    extern "C" {
        fn foo(data: *const c_char);
    }

    pub fn bar(v: &[u8]) {
        let cstr = CString::new(v);
        unsafe { foo(cstr.unwrap().as_ptr()) }
    }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
