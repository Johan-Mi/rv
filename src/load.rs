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
    let mut elf_magic_number = [0; 4];
    if file.read(&mut elf_magic_number)? == 4 && &elf_magic_number == b"\x7fELF"
    {
        file.rewind()?;
        Ok(elf::load_elf_file(
            &::elf::File::open_stream(&mut file).unwrap(),
        ))
    } else {
        // Not an ELF; treat it as a flat binary
        file.rewind()?;
        let mut program_bytes = Vec::new();
        file.read_to_end(&mut program_bytes)?;
        let pc = program_bytes.as_ptr().cast();
        Ok((program_bytes, pc))
    }
}
