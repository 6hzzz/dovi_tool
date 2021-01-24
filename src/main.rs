use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;

mod bits;
use bits::{bitvec_reader, bitvec_writer};

mod dovi;
use dovi::{convert_rpu::RpuConverter, demuxer::Demuxer, rpu_extractor::RpuExtractor, Format};

pub mod cli;
use cli::{Command, Opt};

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Demux {
            input,
            stdin,
            bl_out,
            el_out,
        } => {
            demux(input, stdin, bl_out, el_out, opt.mode);
        }
        Command::ExtractRpu {
            input,
            stdin,
            rpu_out,
        } => {
            extract_rpu(input, stdin, rpu_out, opt.mode);
        }
        Command::ConvertRpu {
            input,
            stdin,
            output,
            discard_el,
        } => {
            convert_rpu(input, stdin, output, opt.mode, discard_el);
        }
    }

    Ok(())
}

fn input_format(input: &PathBuf) -> Result<Format, &str> {
    let regex = Regex::new(r"\.(hevc|.?265|mkv)").unwrap();
    let file_name = match input.file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => "",
    };

    if file_name == "-" {
        Ok(Format::RawStdin)
    } else if regex.is_match(file_name) && input.is_file() {
        if file_name.contains("mkv") {
            Ok(Format::Matroska)
        } else {
            Ok(Format::Raw)
        }
    } else if file_name == "" {
        Err("Missing input.")
    } else if !input.is_file() {
        Err("Input file doesn't exist.")
    } else {
        Err("Invalid input file type.")
    }
}

fn demux(
    input: Option<PathBuf>,
    stdin: Option<PathBuf>,
    bl_out: Option<PathBuf>,
    el_out: Option<PathBuf>,
    mode: Option<u8>,
) {
    let input = match input {
        Some(input) => input,
        None => match stdin {
            Some(stdin) => stdin,
            None => PathBuf::new(),
        },
    };

    match input_format(&input) {
        Ok(format) => {
            let bl_out = match bl_out {
                Some(path) => path,
                None => PathBuf::from("BL.hevc"),
            };

            let el_out = match el_out {
                Some(path) => path,
                None => PathBuf::from("EL.hevc"),
            };

            let demuxer = Demuxer::new(format, input, bl_out, el_out);
            demuxer.process_input(mode);
        }
        Err(msg) => println!("{}", msg),
    }
}

fn extract_rpu(
    input: Option<PathBuf>,
    stdin: Option<PathBuf>,
    rpu_out: Option<PathBuf>,
    mode: Option<u8>,
) {
    let input = match input {
        Some(input) => input,
        None => match stdin {
            Some(stdin) => stdin,
            None => PathBuf::new(),
        },
    };

    match input_format(&input) {
        Ok(format) => {
            let rpu_out = match rpu_out {
                Some(path) => path,
                None => PathBuf::from("RPU.bin"),
            };

            let parser = RpuExtractor::new(format, input, rpu_out);
            parser.process_input(mode);
        }
        Err(msg) => println!("{}", msg),
    }
}

fn convert_rpu(
    input: Option<PathBuf>,
    stdin: Option<PathBuf>,
    output: Option<PathBuf>,
    mode: Option<u8>,
    discard_el: bool,
) {
    let input = match input {
        Some(input) => input,
        None => match stdin {
            Some(stdin) => stdin,
            None => PathBuf::new(),
        },
    };

    match input_format(&input) {
        Ok(format) => {
            let output = match output {
                Some(path) => path,
                None => PathBuf::from("ConvertedRPU.hevc"),
            };

            let parser = RpuConverter::new(format, input, output);
            parser.process_input(mode, discard_el);
        }
        Err(msg) => println!("{}", msg),
    }
}
