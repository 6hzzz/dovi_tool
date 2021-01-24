use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(
        name = "mode",
        short = "m",
        long,
        help = "Sets the mode for RPU processing. --help for more info",
        long_help = "Sets the mode for RPU processing.\nMode 1: Converts the RPU to be MEL compatible\nMode 2: Converts the RPU to be profile 8.1 compatible"
    )]
    pub mode: Option<u8>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "dovi_tool", about = "Stuff about Dolby Vision")]
pub enum Command {
    Demux {
        #[structopt(
            name = "input",
            short = "i",
            long,
            help = "Sets the input file to use",
            conflicts_with = "stdin",
            parse(from_os_str)
        )]
        input: Option<PathBuf>,

        #[structopt(
            help = "Uses stdin as input data",
            conflicts_with = "input",
            parse(from_os_str)
        )]
        stdin: Option<PathBuf>,

        #[structopt(
            short = "bl",
            long,
            help = "BL output file location",
            parse(from_os_str)
        )]
        bl_out: Option<PathBuf>,

        #[structopt(
            short = "el",
            long,
            help = "EL output file location",
            parse(from_os_str)
        )]
        el_out: Option<PathBuf>,
    },

    ExtractRpu {
        #[structopt(
            name = "input",
            short = "i",
            long,
            help = "Sets the input file to use",
            conflicts_with = "stdin",
            parse(from_os_str)
        )]
        input: Option<PathBuf>,

        #[structopt(
            help = "Uses stdin as input data",
            conflicts_with = "input",
            parse(from_os_str)
        )]
        stdin: Option<PathBuf>,

        #[structopt(
            short = "rpu",
            long,
            help = "RPU output file location",
            parse(from_os_str)
        )]
        rpu_out: Option<PathBuf>,
    },

    ConvertRpu {
        #[structopt(
            name = "input",
            short = "i",
            long,
            help = "Sets the input file to use",
            conflicts_with = "stdin",
            parse(from_os_str)
        )]
        input: Option<PathBuf>,

        #[structopt(
            help = "Uses stdin as input data",
            conflicts_with = "input",
            parse(from_os_str)
        )]
        stdin: Option<PathBuf>,

        #[structopt(short = "o", long, help = "Output file location", parse(from_os_str))]
        output: Option<PathBuf>,

        #[structopt(
            name = "discard_el",
            short = "d",
            long,
            help = "Discards the EL NAL units"
        )]
        discard_el: bool,
    },
}
