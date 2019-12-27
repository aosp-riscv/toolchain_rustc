use super::*;
use crate::rustfmt_diff::{make_diff, ModifiedLines};
use std::io::Write;

#[derive(Debug, Default)]
pub(crate) struct ModifiedLinesEmitter;

impl Emitter for ModifiedLinesEmitter {
    fn emit_formatted_file(
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        &self,
=======
        &mut self,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        output: &mut dyn Write,
        FormattedFile {
            original_text,
            formatted_text,
            ..
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        const CONTEXT_SIZE: usize = 0;
        let mismatch = make_diff(original_text, formatted_text, CONTEXT_SIZE);
        let has_diff = !mismatch.is_empty();
        write!(output, "{}", ModifiedLines::from(mismatch))?;
        Ok(EmitterResult { has_diff })
    }
}
