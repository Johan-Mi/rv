/// Creates a mask that extracts a given number of the lowest bits of a [`u32`].
///
/// [`u32`]: std::primitive::u32
pub const fn u32_mask(length: u8) -> u32 {
    (1 << length) - 1
}

/// Creates a mask that extracts a given number of the lowest bits of a [`u16`].
///
/// [`u16`]: std::primitive::u16
pub const fn u16_mask(length: u8) -> u16 {
    (1 << length) - 1
}

/// Shift right, mask, shift left.
pub const fn u32_sms(word: u32, right: u8, length: u8, left: u8) -> u32 {
    ((word >> right) & u32_mask(length)) << left
}

/// Shift right, mask, shift left.
pub const fn u16_sms(word: u16, right: u8, length: u8, left: u8) -> u16 {
    ((word >> right) & u16_mask(length)) << left
}

pub trait SignExtend<T> {
    fn sign_extend(self) -> T;
}

impl SignExtend<u64> for i32 {
    fn sign_extend(self) -> u64 {
        i64::from(self) as u64
    }
}

impl SignExtend<u64> for u32 {
    fn sign_extend(self) -> u64 {
        i64::from(self as i32) as u64
    }
}

impl SignExtend<i64> for u32 {
    fn sign_extend(self) -> i64 {
        i64::from(self as i32)
    }
}

impl SignExtend<i32> for u16 {
    fn sign_extend(self) -> i32 {
        i32::from(self as i16)
    }
}

impl SignExtend<u32> for u16 {
    fn sign_extend(self) -> u32 {
        i32::from(self as i16) as u32
    }
}

impl SignExtend<u32> for i16 {
    fn sign_extend(self) -> u32 {
        i32::from(self) as u32
    }
}
