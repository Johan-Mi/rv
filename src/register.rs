use std::fmt;

use crate::bits::{u16_sms, u32_sms};

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl fmt::Debug for RegisterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            [
                "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1",
                "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3",
                "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
                "t5", "t6",
            ][usize::from(self.0)],
        )
    }
}

impl From<RegisterName> for usize {
    fn from(reg: RegisterName) -> Self {
        reg.0.into()
    }
}
