// compile-flags:-C panic=abort

#![no_std]
#![no_main]

use core::alloc::Layout;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
#[alloc_error_handler] //~ ERROR `#[alloc_error_handler]` is an unstable feature
=======
#[alloc_error_handler] //~ ERROR the `#[alloc_error_handler]` attribute is an experimental feature
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn oom(info: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
