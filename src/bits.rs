/// Creates a mask that extracts a given number of the lowest bits of a [`u32`].
///
/// [`u32`]: std::primitive::u32
pub const fn u32_mask(length: u8) -> u32 {
    (1 << length) - 1
}

/// Shift right, mask, shift left.
pub const fn u32_sms(word: u32, right: u8, length: u8, left: u8) -> u32 {
    ((word >> right) & u32_mask(length)) << left
}
