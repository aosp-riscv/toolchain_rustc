use crate::ffi::OsString;
use crate::marker::PhantomData;
use crate::os::wasi::ffi::OsStringExt;
use crate::vec;

use ::wasi::wasi_unstable as wasi;

pub unsafe fn init(_argc: isize, _argv: *const *const u8) {
}

pub unsafe fn cleanup() {
}

pub struct Args {
    iter: vec::IntoIter<OsString>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

/// Returns the command line arguments
pub fn args() -> Args {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    maybe_args().unwrap_or_else(|_| {
        Args {
            iter: Vec::new().into_iter(),
            _dont_send_or_sync_me: PhantomData
        }
    })
}

fn maybe_args() -> io::Result<Args> {
    unsafe {
        let (mut argc, mut argv_buf_size) = (0, 0);
        cvt_wasi(libc::__wasi_args_sizes_get(&mut argc, &mut argv_buf_size))?;

        let mut argc = vec![core::ptr::null_mut::<libc::c_char>(); argc];
        let mut argv_buf = vec![0; argv_buf_size];
        cvt_wasi(libc::__wasi_args_get(argc.as_mut_ptr(), argv_buf.as_mut_ptr()))?;

        let args = argc.into_iter()
            .map(|ptr| CStr::from_ptr(ptr).to_bytes().to_vec())
            .map(|bytes| OsString::from_vec(bytes))
            .collect::<Vec<_>>();
        Ok(Args {
            iter: args.into_iter(),
            _dont_send_or_sync_me: PhantomData,
        })
=======
    let buf = wasi::args_sizes_get().and_then(|args_sizes| {
        let mut buf = Vec::with_capacity(args_sizes.get_count());
        wasi::args_get(args_sizes, |arg| {
            let arg = OsString::from_vec(arg.to_vec());
            buf.push(arg);
        })?;
        Ok(buf)
    }).unwrap_or(vec![]);
    Args {
        iter: buf.into_iter(),
        _dont_send_or_sync_me: PhantomData
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }
}

impl Args {
    pub fn inner_debug(&self) -> &[OsString] {
        self.iter.as_slice()
    }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        self.iter.next_back()
    }
}
