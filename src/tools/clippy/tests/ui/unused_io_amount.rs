#![allow(dead_code)]
#![warn(clippy::unused_io_amount)]

use std::io;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
fn try_macro<T: io::Read + io::Write>(s: &mut T) -> io::Result<()> {
    r#try!(s.write(b"test"));
    let mut buf = [0u8; 4];
    r#try!(s.read(&mut buf));
    Ok(())
}

=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn question_mark<T: io::Read + io::Write>(s: &mut T) -> io::Result<()> {
    s.write(b"test")?;
    let mut buf = [0u8; 4];
    s.read(&mut buf)?;
    Ok(())
}

fn unwrap<T: io::Read + io::Write>(s: &mut T) {
    s.write(b"test").unwrap();
    let mut buf = [0u8; 4];
    s.read(&mut buf).unwrap();
}

fn main() {}
