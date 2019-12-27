use super::*;
use crate::config::Verbosity;
use std::io::Write;

#[derive(Debug)]
pub(crate) struct StdoutEmitter {
    verbosity: Verbosity,
}

impl StdoutEmitter {
    pub(crate) fn new(verbosity: Verbosity) -> Self {
        Self { verbosity }
    }
}

impl Emitter for StdoutEmitter {
    fn emit_formatted_file(
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        &self,
=======
        &mut self,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        output: &mut dyn Write,
        FormattedFile {
            filename,
            formatted_text,
            ..
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        if self.verbosity != Verbosity::Quiet {
            writeln!(output, "{}:\n", filename)?;
        }
        write!(output, "{}", formatted_text)?;
        Ok(EmitterResult::default())
    }
}
