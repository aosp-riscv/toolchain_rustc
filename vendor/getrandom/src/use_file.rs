// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementations that just need to read from a file
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
extern crate std;

use crate::util_libc::{last_os_error, LazyFd};
use crate::Error;
use core::mem::ManuallyDrop;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::{fs::File, io::Read};

#[cfg(target_os = "redox")]
const FILE_PATH: &str = "rand:";
#[cfg(any(target_os = "android", target_os = "linux", target_os = "netbsd"))]
const FILE_PATH: &str = "/dev/urandom";
#[cfg(any(
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "macos",
    target_os = "solaris",
    target_os = "illumos"
))]
const FILE_PATH: &str = "/dev/random";

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    static FD: LazyFd = LazyFd::new();
    let fd = FD.init(init_file).ok_or(last_os_error())?;
    let file = ManuallyDrop::new(unsafe { File::from_raw_fd(fd) });
    let mut file_ref: &File = &file;

    if cfg!(target_os = "emscripten") {
        // `Crypto.getRandomValues` documents `dest` should be at most 65536 bytes.
        for chunk in dest.chunks_mut(65536) {
            file_ref.read_exact(chunk)?;
        }
    } else {
        file_ref.read_exact(dest)?;
    }
    Ok(())
}

fn init_file() -> Option<RawFd> {
    if FILE_PATH == "/dev/urandom" {
        // read one byte from "/dev/random" to ensure that OS RNG has initialized
        File::open("/dev/random")
            .ok()?
            .read_exact(&mut [0u8; 1])
            .ok()?;
    }
    Some(File::open(FILE_PATH).ok()?.into_raw_fd())
=======
use crate::util_libc::{last_os_error, open_readonly, sys_fill_exact, LazyFd};
use crate::Error;

#[cfg(target_os = "redox")]
const FILE_PATH: &str = "rand:\0";
#[cfg(any(
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "haiku",
    target_os = "macos",
    target_os = "solaris",
    target_os = "illumos"
))]
const FILE_PATH: &str = "/dev/random\0";

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    static FD: LazyFd = LazyFd::new();
    let fd = FD.init(init_file).ok_or_else(last_os_error)?;
    let read = |buf: &mut [u8]| unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };

    if cfg!(target_os = "emscripten") {
        // `Crypto.getRandomValues` documents `dest` should be at most 65536 bytes.
        for chunk in dest.chunks_mut(65536) {
            sys_fill_exact(chunk, read)?;
        }
    } else {
        sys_fill_exact(dest, read)?;
    }
    Ok(())
}

cfg_if! {
    if #[cfg(any(target_os = "android", target_os = "linux", target_os = "netbsd"))] {
        fn init_file() -> Option<libc::c_int> {
            // Poll /dev/random to make sure it is ok to read from /dev/urandom.
            let mut pfd = libc::pollfd {
                fd: unsafe { open_readonly("/dev/random\0")? },
                events: libc::POLLIN,
                revents: 0,
            };

            let ret = loop {
                // A negative timeout means an infinite timeout.
                let res = unsafe { libc::poll(&mut pfd, 1, -1) };
                if res == 1 {
                    break unsafe { open_readonly("/dev/urandom\0") };
                } else if res < 0 {
                    let e = last_os_error().raw_os_error();
                    if e == Some(libc::EINTR) || e == Some(libc::EAGAIN) {
                        continue;
                    }
                }
                // We either hard failed, or poll() returned the wrong pfd.
                break None;
            };
            unsafe { libc::close(pfd.fd) };
            ret
        }
    } else {
        fn init_file() -> Option<libc::c_int> {
            unsafe { open_readonly(FILE_PATH) }
        }
    }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
