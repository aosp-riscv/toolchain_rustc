// ignore-emscripten

fn main() {
    unsafe {
        asm!("");
        //~^ ERROR inline assembly is not stable enough
        llvm_asm!("");
        //~^ ERROR prefer using the new asm! syntax instead
    }
}
