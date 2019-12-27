use super::*;
use std::fs;

#[derive(Debug, Default)]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
pub(crate) struct FilesEmitter;

impl Emitter for FilesEmitter {
    fn emit_formatted_file(
        &self,
        _output: &mut dyn Write,
        FormattedFile {
            filename,
            original_text,
            formatted_text,
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        // Write text directly over original file if there is a diff.
        let filename = ensure_real_path(filename);
        if original_text != formatted_text {
            fs::write(filename, formatted_text)?;
=======
pub(crate) struct FilesEmitter {
    print_misformatted_file_names: bool,
}

impl FilesEmitter {
    pub(crate) fn new(print_misformatted_file_names: bool) -> Self {
        Self {
            print_misformatted_file_names,
        }
    }
}

impl Emitter for FilesEmitter {
    fn emit_formatted_file(
        &mut self,
        output: &mut dyn Write,
        FormattedFile {
            filename,
            original_text,
            formatted_text,
        }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        // Write text directly over original file if there is a diff.
        let filename = ensure_real_path(filename);
        if original_text != formatted_text {
            fs::write(filename, formatted_text)?;
            if self.print_misformatted_file_names {
                writeln!(output, "{}", filename.display())?;
            }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        }
        Ok(EmitterResult::default())
    }
}
