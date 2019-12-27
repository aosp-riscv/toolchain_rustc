/// Common code for printing the backtrace in the same way across the different
/// supported platforms.

use crate::env;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
use crate::fmt;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
use crate::io;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use crate::io::prelude::*;
use crate::mem;
use crate::path::{self, Path};
use crate::ptr;
use crate::sync::atomic::{self, Ordering};
=======
use crate::borrow::Cow;
use crate::io::prelude::*;
use crate::path::{self, Path, PathBuf};
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
use crate::sys::mutex::Mutex;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use backtrace::{BytesOrWideString, Frame, Symbol};

pub const HEX_WIDTH: usize = 2 + 2 * mem::size_of::<usize>();
=======
use backtrace_rs::{BacktraceFmt, BytesOrWideString, PrintFmt};
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

/// Max number of frames to print.
const MAX_NB_FRAMES: usize = 100;

pub fn lock() -> impl Drop {
    struct Guard;
    static LOCK: Mutex = Mutex::new();

    impl Drop for Guard {
        fn drop(&mut self) {
            unsafe {
                LOCK.unlock();
            }
        }
    }

    unsafe {
        LOCK.lock();
        return Guard;
    }
}

/// Prints the current backtrace.
pub fn print(w: &mut dyn Write, format: PrintFmt) -> io::Result<()> {
    // There are issues currently linking libbacktrace into tests, and in
    // general during libstd's own unit tests we're not testing this path. In
    // test mode immediately return here to optimize away any references to the
    // libbacktrace symbols
    if cfg!(test) {
        return Ok(());
    }

    // Use a lock to prevent mixed output in multithreading context.
    // Some platforms also requires it, like `SymFromAddr` on Windows.
    unsafe {
        let _lock = lock();
        _print(w, format)
    }
}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
fn _print(w: &mut dyn Write, format: PrintFormat) -> io::Result<()> {
    writeln!(w, "stack backtrace:")?;

    let mut printer = Printer::new(format, w);
    unsafe {
        backtrace::trace_unsynchronized(|frame| {
            let mut hit = false;
            backtrace::resolve_frame_unsynchronized(frame, |symbol| {
                hit = true;
                printer.output(frame, Some(symbol));
            });
            if !hit {
                printer.output(frame, None);
            }
            !printer.done
        });
=======
unsafe fn _print(w: &mut dyn Write, format: PrintFmt) -> io::Result<()> {
    struct DisplayBacktrace {
        format: PrintFmt,
    }
    impl fmt::Display for DisplayBacktrace {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            unsafe {
                _print_fmt(fmt, self.format)
            }
        }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    if printer.skipped {
        writeln!(
            w,
            "note: Some details are omitted, \
             run with `RUST_BACKTRACE=full` for a verbose backtrace."
        )?;
    }
    Ok(())
=======
    write!(w, "{}", DisplayBacktrace { format })
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
unsafe fn _print_fmt(fmt: &mut fmt::Formatter<'_>, print_fmt: PrintFmt) -> fmt::Result {
    let cwd = env::current_dir().ok();
    let mut print_path = move |fmt: &mut fmt::Formatter<'_>, bows: BytesOrWideString<'_>| {
        output_filename(fmt, bows, print_fmt, cwd.as_ref())
    };
    let mut bt_fmt = BacktraceFmt::new(fmt, print_fmt, &mut print_path);
    bt_fmt.add_context()?;
    let mut idx = 0;
    let mut res = Ok(());
    backtrace_rs::trace_unsynchronized(|frame| {
        if print_fmt == PrintFmt::Short && idx > MAX_NB_FRAMES {
            return false;
        }

        let mut hit = false;
        let mut stop = false;
        backtrace_rs::resolve_frame_unsynchronized(frame, |symbol| {
            hit = true;
            if print_fmt == PrintFmt::Short {
                if let Some(sym) = symbol.name().and_then(|s| s.as_str()) {
                    if sym.contains("__rust_begin_short_backtrace") {
                        stop = true;
                        return;
                    }
                }
            }

            res = bt_fmt.frame().symbol(frame, symbol);
        });
        if stop {
            return false;
        }
        if !hit {
            res = bt_fmt.frame().print_raw(frame.ip(), None, None, None);
        }

        idx += 1;
        res.is_ok()
    });
    res?;
    bt_fmt.finish()?;
    if print_fmt == PrintFmt::Short {
        writeln!(
            fmt,
            "note: Some details are omitted, \
             run with `RUST_BACKTRACE=full` for a verbose backtrace."
        )?;
    }
    Ok(())
}

>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
/// Fixed frame used to clean the backtrace with `RUST_BACKTRACE=1`.
#[inline(never)]
pub fn __rust_begin_short_backtrace<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
    F: Send,
    T: Send,
{
    f()
}

// For now logging is turned off by default, and this function checks to see
// whether the magical environment variable is present to see if it's turned on.
pub fn log_enabled() -> Option<PrintFmt> {
    use crate::sync::atomic::{self, Ordering};

    // Setting environment variables for Fuchsia components isn't a standard
    // or easily supported workflow. For now, always display backtraces.
    if cfg!(target_os = "fuchsia") {
        return Some(PrintFmt::Full);
    }

    static ENABLED: atomic::AtomicIsize = atomic::AtomicIsize::new(0);
    match ENABLED.load(Ordering::SeqCst) {
        0 => {}
        1 => return None,
        2 => return Some(PrintFmt::Short),
        _ => return Some(PrintFmt::Full),
    }

    let val = env::var_os("RUST_BACKTRACE").and_then(|x| {
        if &x == "0" {
            None
        } else if &x == "full" {
            Some(PrintFmt::Full)
        } else {
            Some(PrintFmt::Short)
        }
    });
    ENABLED.store(
        match val {
            Some(v) => v as isize,
            None => 1,
        },
        Ordering::SeqCst,
    );
    val
}

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
struct Printer<'a, 'b> {
    format: PrintFormat,
    done: bool,
    skipped: bool,
    idx: usize,
    out: &'a mut (dyn Write + 'b),
}

impl<'a, 'b> Printer<'a, 'b> {
    fn new(format: PrintFormat, out: &'a mut (dyn Write + 'b)) -> Printer<'a, 'b> {
        Printer { format, done: false, skipped: false, idx: 0, out }
    }

    /// Prints the symbol of the backtrace frame.
    ///
    /// These output functions should now be used everywhere to ensure consistency.
    /// You may want to also use `output_fileline`.
    fn output(&mut self, frame: &Frame, symbol: Option<&Symbol>) {
        if self.idx > MAX_NB_FRAMES {
            self.done = true;
            self.skipped = true;
            return;
        }
        if self._output(frame, symbol).is_err() {
            self.done = true;
        }
        self.idx += 1;
    }

    fn _output(&mut self, frame: &Frame, symbol: Option<&Symbol>) -> io::Result<()> {
        if self.format == PrintFormat::Short {
            if let Some(sym) = symbol.and_then(|s| s.name()).and_then(|s| s.as_str()) {
                if sym.contains("__rust_begin_short_backtrace") {
                    self.skipped = true;
                    self.done = true;
                    return Ok(());
                }
            }

            // Remove the `17: 0x0 - <unknown>` line.
            if self.format == PrintFormat::Short && frame.ip() == ptr::null_mut() {
                self.skipped = true;
                return Ok(());
            }
        }

        match self.format {
            PrintFormat::Full => {
                write!(self.out, "  {:2}: {:2$?} - ", self.idx, frame.ip(), HEX_WIDTH)?
            }
            PrintFormat::Short => write!(self.out, "  {:2}: ", self.idx)?,
        }

        match symbol.and_then(|s| s.name()) {
            Some(symbol) => {
                match self.format {
                    PrintFormat::Full => write!(self.out, "{}", symbol)?,
                    // Strip the trailing hash if short mode.
                    PrintFormat::Short => write!(self.out, "{:#}", symbol)?,
                }
            }
            None => self.out.write_all(b"<unknown>")?,
        }
        self.out.write_all(b"\n")?;
        if let Some(sym) = symbol {
            self.output_fileline(sym)?;
        }
        Ok(())
    }

    /// Prints the filename and line number of the backtrace frame.
    ///
    /// See also `output`.
    fn output_fileline(&mut self, symbol: &Symbol) -> io::Result<()> {
        #[cfg(windows)]
        let path_buf;
        let file = match symbol.filename_raw() {
            #[cfg(unix)]
            Some(BytesOrWideString::Bytes(bytes)) => {
                use crate::os::unix::prelude::*;
                Path::new(crate::ffi::OsStr::from_bytes(bytes))
            }
            #[cfg(not(unix))]
            Some(BytesOrWideString::Bytes(bytes)) => {
                Path::new(crate::str::from_utf8(bytes).unwrap_or("<unknown>"))
            }
            #[cfg(windows)]
            Some(BytesOrWideString::Wide(wide)) => {
                use crate::os::windows::prelude::*;
                path_buf = crate::ffi::OsString::from_wide(wide);
                Path::new(&path_buf)
            }
            #[cfg(not(windows))]
            Some(BytesOrWideString::Wide(_wide)) => {
                Path::new("<unknown>")
            }
            None => return Ok(()),
        };
        let line = match symbol.lineno() {
            Some(line) => line,
            None => return Ok(()),
        };
        // prior line: "  ##: {:2$} - func"
        self.out.write_all(b"")?;
        match self.format {
            PrintFormat::Full => write!(self.out, "           {:1$}", "", HEX_WIDTH)?,
            PrintFormat::Short => write!(self.out, "           ")?,
        }

        let mut already_printed = false;
        if self.format == PrintFormat::Short && file.is_absolute() {
            if let Ok(cwd) = env::current_dir() {
                if let Ok(stripped) = file.strip_prefix(&cwd) {
                    if let Some(s) = stripped.to_str() {
                        write!(self.out, "  at .{}{}:{}", path::MAIN_SEPARATOR, s, line)?;
                        already_printed = true;
                    }
=======
/// Prints the filename of the backtrace frame.
///
/// See also `output`.
pub fn output_filename(
    fmt: &mut fmt::Formatter<'_>,
    bows: BytesOrWideString<'_>,
    print_fmt: PrintFmt,
    cwd: Option<&PathBuf>,
) -> fmt::Result {
    let file: Cow<'_, Path> = match bows {
        #[cfg(unix)]
        BytesOrWideString::Bytes(bytes) => {
            use crate::os::unix::prelude::*;
            Path::new(crate::ffi::OsStr::from_bytes(bytes)).into()
        }
        #[cfg(not(unix))]
        BytesOrWideString::Bytes(bytes) => {
            Path::new(crate::str::from_utf8(bytes).unwrap_or("<unknown>")).into()
        }
        #[cfg(windows)]
        BytesOrWideString::Wide(wide) => {
            use crate::os::windows::prelude::*;
            Cow::Owned(crate::ffi::OsString::from_wide(wide).into())
        }
        #[cfg(not(windows))]
        BytesOrWideString::Wide(_wide) => {
            Path::new("<unknown>").into()
        }
    };
    if print_fmt == PrintFmt::Short && file.is_absolute() {
        if let Some(cwd) = cwd {
            if let Ok(stripped) = file.strip_prefix(&cwd) {
                if let Some(s) = stripped.to_str() {
                    return write!(fmt, ".{}{}", path::MAIN_SEPARATOR, s);
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
                }
            }
        }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        if !already_printed {
            write!(self.out, "  at {}:{}", file.display(), line)?;
        }

        self.out.write_all(b"\n")
    }
=======
    }
    fmt::Display::fmt(&file.display(), fmt)
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}
