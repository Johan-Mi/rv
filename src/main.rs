#![allow(clippy::unusual_byte_groupings)]
#![deny(unsafe_op_in_unsafe_fn)]

mod bits;
mod cpu;
mod error;
mod instruction;
mod load;
mod register;

use cpu::Cpu;
use gumdrop::Options;
use std::path::PathBuf;

#[derive(Options)]
pub struct Opts {
    /// Display this message
    help: bool,

    /// ELF or flat binary to execute
    #[options(free, required)]
    file: PathBuf,

    /// Print extra debug information
    verbose: bool,
}

fn main() {
    if let Err(err) = (|| {
        let opts = Opts::parse_args_default_or_exit();
        let (program_bytes, pc) = load::load_program(&opts.file)?;
        let mut cpu = Cpu::new(opts, pc);
        unsafe {
            cpu.run()?;
        }
        // Keep the program allocated while the CPU is running
        drop(program_bytes);

        Ok::<(), Box<dyn std::error::Error>>(())
    })() {
        eprintln!("{err}");
    }
}
