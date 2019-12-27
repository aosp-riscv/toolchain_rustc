<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use crate::spec::{LinkArgs, LinkerFlavor, TargetOptions, RelroLevel};
use std::default::Default;

pub fn opts() -> TargetOptions {
    let mut args = LinkArgs::new();
    args.insert(LinkerFlavor::Gcc, vec![
        // We want to be able to strip as much executable code as possible
        // from the linker command line, and this flag indicates to the
        // linker that it can avoid linking in dynamic libraries that don't
        // actually satisfy any symbols up to that point (as with many other
        // resolutions the linker does). This option only applies to all
        // following libraries so we're sure to pass it as one of the first
        // arguments.
        "-Wl,--as-needed".to_string(),

        // Always enable NX protection when it is available
        "-Wl,-z,noexecstack".to_string(),
    ]);

    let mut late_lk_args = LinkArgs::new();
    late_lk_args.insert(LinkerFlavor::Gcc, vec![
        "-lnet".to_string(),
        "-lunix".to_string(),
    ]);

    TargetOptions {
        linker: Some("vx-cxx".to_string()),
        exe_suffix: ".vxe".to_string(),
        late_link_args: late_lk_args,
        dynamic_linking: true,
        executables: true,
        target_family: Some("unix".to_string()),
        linker_is_gnu: true,
        has_rpath: true,
        pre_link_args: args,
        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        has_elf_tls: true,
=======
use crate::spec::{LinkArgs, LinkerFlavor, TargetOptions};
use std::default::Default;

pub fn opts() -> TargetOptions {
    let mut args_crt = LinkArgs::new();
    args_crt.insert(LinkerFlavor::Gcc, vec![
        "--static-crt".to_string(),
    ]);
    let mut args = LinkArgs::new();
    args.insert(LinkerFlavor::Gcc, vec![
        // We want to be able to strip as much executable code as possible
        // from the linker command line, and this flag indicates to the
        // linker that it can avoid linking in dynamic libraries that don't
        // actually satisfy any symbols up to that point (as with many other
        // resolutions the linker does). This option only applies to all
        // following libraries so we're sure to pass it as one of the first
        // arguments.
        "-Wl,--as-needed".to_string(),
    ]);

    TargetOptions {
        linker: Some("wr-c++".to_string()),
        exe_suffix: ".vxe".to_string(),
        dynamic_linking: true,
        executables: true,
        target_family: Some("unix".to_string()),
        linker_is_gnu: true,
        has_rpath: true,
        pre_link_args: args,
        position_independent_executables: false,
        has_elf_tls: true,
        pre_link_args_crt: args_crt,
        crt_static_default: true,
        crt_static_respected: true,
        crt_static_allows_dylibs: true,
        // VxWorks needs to implement this to support profiling
        target_mcount: "_mcount".to_string(),
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        .. Default::default()
    }
}
