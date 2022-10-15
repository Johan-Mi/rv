pub fn load_elf_file(file: &elf::File) -> (Vec<u8>, *const u16) {
    // Sections with an address of zero should not be loaded
    let relevant_sections = || {
        file.sections
            .iter()
            .filter(|section| section.shdr.addr != 0)
    };

    // Save some memory by moving all of the section starting addresses so that
    // the lowest one ends up at zero
    let address_offset = relevant_sections()
        .map(|section| section.shdr.addr)
        .min()
        .unwrap_or_default();

    let total_size = relevant_sections()
        .map(|section| section.shdr.addr - address_offset + section.shdr.size)
        .max()
        .unwrap_or_default() as usize;
    let mut bytes = vec![0u8; total_size];

    for section in relevant_sections() {
        bytes[(section.shdr.addr - address_offset) as usize..]
            [..section.shdr.size as usize]
            .copy_from_slice(&section.data);
    }

    // Convert the entry address requested in the ELF to where it actually gets
    // allocated
    let entry = bytes
        .as_ptr()
        .wrapping_add((file.ehdr.entry - address_offset) as usize)
        .cast();

    (bytes, entry)
}
