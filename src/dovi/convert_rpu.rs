use std::path::PathBuf;

use super::{io, Format};
use indicatif::ProgressBar;

use io::{DoviReader, DoviWriter, WriterOptions};

pub struct RpuConverter {
    format: Format,
    input: PathBuf,
    output: PathBuf,
}

impl RpuConverter {
    pub fn new(format: Format, input: PathBuf, output: PathBuf) -> Self {
        Self {
            format,
            input,
            output,
        }
    }

    pub fn process_input(&self, mode: Option<u8>, discard_el: bool) {
        let pb = super::initialize_progress_bar(&self.format, &self.input);

        match self.format {
            Format::Matroska => panic!("unsupported"),
            _ => self.convert_rpu(Some(&pb), mode, discard_el),
        };
    }

    pub fn convert_rpu(&self, pb: Option<&ProgressBar>, mode: Option<u8>, discard_el: bool) {
        let mut dovi_reader = DoviReader::new(mode);
        let writer_opts = WriterOptions {
            full_out: Some(self.output.clone()),
            discard_el,
            ..Default::default()
        };

        let mut dovi_writer = DoviWriter::new(writer_opts);

        match dovi_reader.read_write_from_io(&self.format, &self.input, pb, &mut dovi_writer) {
            Ok(_) => (),
            Err(e) => panic!(e),
        }
    }
}
