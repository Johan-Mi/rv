use elf::types::PT_LOAD;
use std::io::Cursor;

pub fn load_elf_file(raw_file: &[u8]) -> (Vec<u8>, *const u16) {
    let file = elf::File::open_stream(&mut Cursor::new(raw_file)).unwrap();

    // Save some memory by moving all of the segment starting addresses so that
    // the lowest one ends up at zero
    let address_offset = file
        .phdrs
        .iter()
        .map(|segment| segment.vaddr)
        .min()
        .unwrap_or_default();

    let total_size = file
        .phdrs
        .iter()
        .map(|segment| segment.vaddr - address_offset + segment.memsz)
        .max()
        .unwrap_or_default() as usize;
    let mut bytes = vec![0u8; total_size];

    for segment in file.phdrs.iter() {
        match segment.progtype {
            PT_LOAD => {
                bytes[(segment.vaddr - address_offset) as usize..]
                    [..segment.filesz as usize]
                    .copy_from_slice(
                        &raw_file[segment.offset as usize..]
                            [..segment.filesz as usize],
                    );
            }
            _ => {}
        }
    }

    // Convert the entry address requested in the ELF to where it actually gets
    // allocated
    let entry = bytes
        .as_ptr()
        .wrapping_add((file.ehdr.entry - address_offset) as usize)
        .cast();

    (bytes, entry)
}
