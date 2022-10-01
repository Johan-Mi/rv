use crate::{
    bits::{u32_mask, u32_sms},
    error::Error,
};

#[derive(Debug)]
pub enum Instruction {
    R {
        funct: RFunct,
        rs2: RegisterName,
        rs1: RegisterName,
        rd: RegisterName,
    },
    I {
        imm: u32,
        rs1: RegisterName,
        funct: IFunct,
        rd: RegisterName,
    },
    S {
        imm: u16,
        rs2: RegisterName,
        rs1: RegisterName,
        funct: SFunct,
    },
    B {
        imm: i16,
        rs2: RegisterName,
        rs1: RegisterName,
        funct: BFunct,
    },
    U {
        imm: u32,
        rd: RegisterName,
        opcode: UOpcode,
    },
    Jal {
        imm: i32,
        rd: RegisterName,
    },
    Ecall,
}

impl TryFrom<u32> for Instruction {
    type Error = Error;

    fn try_from(word: u32) -> Result<Self, Self::Error> {
        let raw_opcode = (word & u32_mask(7)) as u8;
        if word == 0x0000_0073 {
            Ok(Self::Ecall)
        } else {
            match raw_opcode {
                0b011_0011 => Ok(Self::R {
                    funct: RFunct::try_from(word)?,
                    rs2: RegisterName::rs2(word),
                    rs1: RegisterName::rs1(word),
                    rd: RegisterName::rd(word),
                }),
                0b001_0011 | 0b000_0011 => Ok(Self::I {
                    imm: u32_sms(word, 20, 12, 0),
                    rs1: RegisterName::rs1(word),
                    funct: IFunct::try_from(word)?,
                    rd: RegisterName::rd(word),
                }),
                0b010_0011 => Ok(Self::S {
                    imm: (u32_sms(word, 25, 7, 5) | u32_sms(word, 7, 5, 0))
                        as u16,
                    rs2: RegisterName::rs2(word),
                    rs1: RegisterName::rs1(word),
                    funct: SFunct::try_from(word)?,
                }),
                0b110_0011 => Ok(Self::B {
                    imm: (u32_sms(word, 31, 1, 15)
                        | u32_sms(word, 7, 1, 14)
                        | u32_sms(word, 25, 6, 8)
                        | u32_sms(word, 8, 4, 4))
                        as i16
                        >> 3,
                    rs2: RegisterName::rs2(word),
                    rs1: RegisterName::rs1(word),
                    funct: BFunct::try_from(word)?,
                }),
                0b001_0111 => Ok(Self::U {
                    imm: word & (u32_mask(20) << 12),
                    rd: RegisterName::rd(word),
                    opcode: UOpcode::Auipc,
                }),
                0b_1101111 => Ok(Self::Jal {
                    imm: (u32_sms(word, 31, 1, 31)
                        | u32_sms(word, 12, 8, 23)
                        | u32_sms(word, 20, 1, 22)
                        | u32_sms(word, 21, 10, 12))
                        as i32
                        >> 11,
                    rd: RegisterName::rd(word),
                }),
                _ => Err(Error::UnknownInstruction(word)),
            }
        }
    }
}

#[derive(Debug)]
pub enum RFunct {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
}

impl TryFrom<u32> for RFunct {
    type Error = Error;

    fn try_from(word: u32) -> Result<Self, Self::Error> {
        let raw_funct = u32_sms(word, 12, 3, 0) | u32_sms(word, 25, 7, 3);
        match raw_funct {
            0b0000000_000 => Ok(Self::Add),
            0b0100000_000 => Ok(Self::Sub),
            0b0000000_001 => Ok(Self::Sll),
            0b0000000_010 => Ok(Self::Slt),
            0b0000000_011 => Ok(Self::Sltu),
            0b0000000_100 => Ok(Self::Xor),
            0b0000000_101 => Ok(Self::Srl),
            0b0100000_101 => Ok(Self::Sra),
            0b0000000_110 => Ok(Self::Or),
            0b0000000_111 => Ok(Self::And),
            _ => Err(Error::UnknownInstruction(word)),
        }
    }
}

#[derive(Debug)]
pub enum IFunct {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
    Slli,
    Srli,
    Srai,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

impl TryFrom<u32> for IFunct {
    type Error = Error;

    fn try_from(word: u32) -> Result<Self, Self::Error> {
        let raw_funct = u32_sms(word, 12, 3, 0);
        let raw_opcode = (word & u32_mask(7)) as u8;
        let srai_bit = word & 1 << 30 != 0;
        match raw_opcode {
            0b001_0011 => match raw_funct {
                0b000 => Ok(Self::Addi),
                0b010 => Ok(Self::Slti),
                0b011 => Ok(Self::Sltiu),
                0b100 => Ok(Self::Xori),
                0b110 => Ok(Self::Ori),
                0b111 => Ok(Self::Andi),
                0b001 => Ok(Self::Slli),
                0b101 => Ok(if srai_bit { Self::Srai } else { Self::Srli }),
                _ => unreachable!(),
            },
            0b000_0011 => match raw_funct {
                0b000 => Ok(Self::Lb),
                0b001 => Ok(Self::Lh),
                0b010 => Ok(Self::Lw),
                0b100 => Ok(Self::Lbu),
                0b101 => Ok(Self::Lhu),
                _ => Err(Error::UnknownInstruction(word)),
            },
            _ => Err(Error::UnknownInstruction(word)),
        }
    }
}

#[derive(Debug)]
pub enum SFunct {
    Sb,
    Sh,
    Sw,
    Sd,
}

impl TryFrom<u32> for SFunct {
    type Error = Error;

    fn try_from(word: u32) -> Result<Self, Self::Error> {
        let raw_funct = u32_sms(word, 12, 3, 0);
        match raw_funct {
            0b000 => Ok(Self::Sb),
            0b010 => Ok(Self::Sh),
            0b100 => Ok(Self::Sw),
            0b011 => Ok(Self::Sd),
            _ => Err(Error::UnknownInstruction(word)),
        }
    }
}

#[derive(Debug)]
pub enum BFunct {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

impl TryFrom<u32> for BFunct {
    type Error = Error;

    fn try_from(word: u32) -> Result<Self, Self::Error> {
        let raw_funct = u32_sms(word, 12, 3, 0);
        match raw_funct {
            0b000 => Ok(Self::Beq),
            0b001 => Ok(Self::Bne),
            0b100 => Ok(Self::Blt),
            0b101 => Ok(Self::Bge),
            0b110 => Ok(Self::Bltu),
            0b111 => Ok(Self::Bgeu),
            _ => Err(Error::UnknownInstruction(word)),
        }
    }
}

#[derive(Debug)]
pub struct RegisterName(u8);

impl RegisterName {
    const fn rd(word: u32) -> Self {
        Self(u32_sms(word, 7, 5, 0) as u8)
    }

    const fn rs1(word: u32) -> Self {
        Self(u32_sms(word, 15, 5, 0) as u8)
    }

    const fn rs2(word: u32) -> Self {
        Self(u32_sms(word, 20, 5, 0) as u8)
    }
}

impl From<RegisterName> for usize {
    fn from(reg: RegisterName) -> Self {
        reg.0.into()
    }
}

#[derive(Debug)]
pub enum SOpcode {}

#[derive(Debug)]
pub enum UOpcode {
    Lui,
    Auipc,
}

#[derive(Debug)]
pub enum JOpcode {}
