pub fn load_elf_file(file: elf::File) -> (Vec<u8>, *const u16) {
    let total_size = file
        .sections
        .iter()
        .map(|section| section.shdr.offset + section.shdr.size)
        .max()
        // The default value here doesn't matter since it only gets used if
        // there are no sections, which will trigger an error later in this
        // function anyway
        .unwrap_or_default() as usize;
    let mut bytes = vec![0u8; total_size];

    for section in &file.sections {
        bytes[section.shdr.offset as usize..][..section.shdr.size as usize]
            .copy_from_slice(&section.data);
    }

    let text_section = file.get_section(".text").unwrap();
    // Convert the entry address requested in the ELF to where it actually gets
    // allocated by using the `.text` section as a frame of reference
    let entry = bytes
        .as_ptr()
        .wrapping_add(
            file.ehdr
                .entry
                .wrapping_sub(text_section.shdr.addr)
                .wrapping_add(text_section.shdr.offset) as usize,
        )
        .cast();

    (bytes, entry)
}
