mod elf;

use std::{
    fs::File,
    io::{Read, Seek},
    path::Path,
};

pub fn load_program(
    path: &Path,
) -> Result<(Vec<u8>, *const u16), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    match ::elf::File::open_stream(&mut file) {
        Err(::elf::ParseError::InvalidMagic) => {
            // Not an ELF; treat it as a flat binary
            file.rewind()?;
            let mut program_bytes = Vec::new();
            file.read_to_end(&mut program_bytes)?;
            let pc = program_bytes.as_ptr().cast();
            Ok((program_bytes, pc))
        }
        res => Ok(elf::load_elf_file(res.unwrap())),
    }
}
