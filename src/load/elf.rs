pub fn load_elf_file(file: elf::File) -> (Vec<u8>, *const u16) {
    // Save some memory by moving all of the section starting addresses so that
    // the lowest one ends up at zero
    let address_offset = file
        .sections
        .iter()
        .map(|section| section.shdr.addr)
        .filter(|&addr| addr != 0)
        .min()
        .unwrap_or_default();

    let total_size = file
        .sections
        .iter()
        .filter(|section| section.shdr.addr != 0)
        .map(|section| section.shdr.addr - address_offset + section.shdr.size)
        .max()
        // The default value here doesn't matter since it only gets used if
        // there are no sections, which will trigger an error later in this
        // function anyway
        .unwrap_or_default() as usize;
    let mut bytes = vec![0u8; total_size];

    for section in file
        .sections
        .iter()
        .filter(|section| section.shdr.addr != 0)
    {
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
