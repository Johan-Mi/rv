mod elf;

use std::{fs, path::Path};

pub fn load_program(
    path: &Path,
) -> Result<(Vec<u8>, *const u16), Box<dyn std::error::Error>> {
    let raw_program = fs::read(path)?;
    if raw_program.starts_with(b"\x7fELF") {
        Ok(elf::load_elf_file(&raw_program))
    } else {
        // Not an ELF; treat it as a flat binary
        let pc = raw_program.as_ptr().cast();
        Ok((raw_program, pc))
    }
}
