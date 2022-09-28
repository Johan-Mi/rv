#![feature(mixed_integer_ops)]
#![allow(clippy::unusual_byte_groupings)]
#![deny(unsafe_op_in_unsafe_fn)]

mod bits;
mod cpu;
mod error;
mod instruction;

use cpu::Cpu;
use gumdrop::Options;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Options)]
pub struct Opts {
    /// Display this message
    help: bool,

    /// Flat binary to execute
    #[options(free, required)]
    file: PathBuf,

    /// Print extra debug information
    verbose: bool,
}

fn main() {
    if let Err(err) = (|| {
        let opts = Opts::parse_args_default_or_exit();

        let mut file = File::open(&opts.file)?;
        let mut program_bytes = Vec::new();
        file.read_to_end(&mut program_bytes)?;

        let mut cpu = Cpu::new(opts, program_bytes.as_ptr().cast());
        unsafe {
            cpu.run()?;
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    })() {
        eprintln!("{err}");
    }
}
