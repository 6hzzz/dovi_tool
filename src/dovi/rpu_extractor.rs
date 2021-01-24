use std::path::PathBuf;

use super::{io, Format};
use indicatif::ProgressBar;

use io::{DoviReader, DoviWriter, WriterOptions};

pub struct RpuExtractor {
    format: Format,
    input: PathBuf,
    rpu_out: PathBuf,
}

impl RpuExtractor {
    pub fn new(format: Format, input: PathBuf, rpu_out: PathBuf) -> Self {
        Self {
            format,
            input,
            rpu_out,
        }
    }

    pub fn process_input(&self, mode: Option<u8>) {
        let pb = super::initialize_progress_bar(&self.format, &self.input);

        match self.format {
            Format::Matroska => panic!("unsupported"),
            _ => self.extract_rpu_from_el(Some(&pb), mode),
        };
    }

    pub fn extract_rpu_from_el(&self, pb: Option<&ProgressBar>, mode: Option<u8>) {
        let mut dovi_reader = DoviReader::new(mode);
        let writer_opts = WriterOptions {
            rpu_out: Some(self.rpu_out.clone()),
            ..Default::default()
        };

        let mut dovi_writer = DoviWriter::new(writer_opts);

        match dovi_reader.read_write_from_io(&self.format, &self.input, pb, &mut dovi_writer) {
            Ok(_) => (),
            Err(e) => panic!(e),
        }
    }
}
