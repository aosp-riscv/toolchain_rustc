pub use self::process_common::{Command, ExitStatus, ExitCode, Stdio, StdioPipes};
pub use self::process_inner::Process;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)

mod process_common;
#[path = "process_vxworks.rs"]
mod process_inner;
mod rtp;
=======
pub use crate::ffi::OsString as EnvKey;

mod process_common;
#[path = "process_vxworks.rs"]
mod process_inner;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
