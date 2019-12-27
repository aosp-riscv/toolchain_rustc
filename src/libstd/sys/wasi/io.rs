use crate::marker::PhantomData;
use crate::slice;

use ::wasi::wasi_unstable as wasi;
use core::ffi::c_void;

#[repr(transparent)]
pub struct IoSlice<'a> {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    vec: __wasi_ciovec_t,
=======
    vec: wasi::CIoVec,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    _p: PhantomData<&'a [u8]>,
}

impl<'a> IoSlice<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
        IoSlice {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
            vec: __wasi_ciovec_t {
=======
            vec: wasi::CIoVec {
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
                buf: buf.as_ptr() as *const c_void,
                buf_len: buf.len(),
            },
            _p: PhantomData,
        }
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        if self.vec.buf_len < n {
            panic!("advancing IoSlice beyond its length");
        }

        unsafe {
            self.vec.buf_len -= n;
            self.vec.buf = self.vec.buf.add(n);
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.vec.buf as *const u8, self.vec.buf_len)
        }
    }
}

#[repr(transparent)]
pub struct IoSliceMut<'a> {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    vec: __wasi_iovec_t,
=======
    vec: wasi::IoVec,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    _p: PhantomData<&'a mut [u8]>,
}

impl<'a> IoSliceMut<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
        IoSliceMut {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
            vec: __wasi_iovec_t {
=======
            vec: wasi::IoVec {
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
                buf: buf.as_mut_ptr() as *mut c_void,
                buf_len: buf.len()
            },
            _p: PhantomData,
        }
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        if self.vec.buf_len < n {
            panic!("advancing IoSlice beyond its length");
        }

        unsafe {
            self.vec.buf_len -= n;
            self.vec.buf = self.vec.buf.add(n);
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.vec.buf as *const u8, self.vec.buf_len)
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.vec.buf as *mut u8, self.vec.buf_len)
        }
    }
}
