use crate::bits::{u16_sms, u32_sms};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterName(u8);

impl RegisterName {
    pub const X0: Self = Self(0);
    pub const X2: Self = Self(2);

    pub const fn rd(word: u32) -> Self {
        Self(u32_sms(word, 7, 5, 0) as u8)
    }

    pub const fn rs1(word: u32) -> Self {
        Self(u32_sms(word, 15, 5, 0) as u8)
    }

    pub const fn rs2(word: u32) -> Self {
        Self(u32_sms(word, 20, 5, 0) as u8)
    }

    pub const fn compressed_rd(word: u16) -> Self {
        Self(u16_sms(word, 7, 5, 0) as u8)
    }

    pub const fn compressed_rs2(word: u16) -> Self {
        Self(u16_sms(word, 2, 5, 0) as u8)
    }
}

impl From<RegisterName> for usize {
    fn from(reg: RegisterName) -> Self {
        reg.0.into()
    }
}
