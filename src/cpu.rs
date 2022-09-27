use crate::{
    bits::u32_sms,
    error::Result,
    instruction::{IFunct, Instruction, RegisterName, UOpcode},
    Opts,
};
use std::ops::{Index, IndexMut};

pub struct Cpu {
    opts: Opts,
    zero: u64, // Never read from this
    registers: [u64; 31],
    pc: *const u32,
}

impl Cpu {
    pub fn new(opts: Opts, pc: *const u32) -> Self {
        Self {
            opts,
            zero: 0,
            registers: Default::default(),
            pc,
        }
    }

    pub unsafe fn run(&mut self) -> Result<()> {
        loop {
            unsafe { self.step() }?;
        }
    }

    unsafe fn step(&mut self) -> Result<()> {
        let instruction = Instruction::try_from(unsafe { *self.pc })?;
        self.pc = self.pc.wrapping_add(1);
        self.run_instruction(instruction);
        Ok(())
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        if self.opts.verbose {
            eprintln!("Running: {instruction:?}");
        }
        match instruction {
            Instruction::R {
                funct,
                rs2,
                rs1,
                rd,
            } => todo!(),
            Instruction::I {
                imm,
                rs1,
                funct,
                rd,
            } => match funct {
                IFunct::Addi => {
                    self[rd] = self[rs1]
                        .wrapping_add_signed(sign_extend_12bit(imm).into());
                }
                IFunct::Slti => todo!(),
                IFunct::Sltiu => todo!(),
                IFunct::Xori => todo!(),
                IFunct::Ori => todo!(),
                IFunct::Andi => todo!(),
                IFunct::Slli => todo!(),
                IFunct::Srli => todo!(),
                IFunct::Srai => todo!(),
                IFunct::Lb => todo!(),
                IFunct::Lh => todo!(),
                IFunct::Lw => {
                    self[rd] = unsafe {
                        *(self[rs1] as *const i32).wrapping_byte_offset(
                            sign_extend_12bit(imm) as isize,
                        )
                    } as i64 as u64;
                }
                IFunct::Lbu => todo!(),
                IFunct::Lhu => todo!(),
            },
            Instruction::S {
                imm,
                rs2,
                rs1,
                funct,
                opcode,
            } => todo!(),
            Instruction::B {
                imm,
                rs2,
                rs1,
                funct,
            } => todo!(),
            Instruction::U { imm, rd, opcode } => match opcode {
                UOpcode::Lui => self[rd] = imm as i32 as i64 as u64,
                UOpcode::Auipc => {
                    self[rd] = (self.pc as u64)
                        .wrapping_add_signed(imm as i32 as i64)
                        .wrapping_sub(4);
                }
            },
            Instruction::J { imm, rd, opcode } => todo!(),
            Instruction::Ecall => {
                self.registers[9] = unsafe {
                    libc::syscall(
                        self.registers[16] as i64,
                        self.registers[9] as i64,
                        self.registers[10] as i64,
                        self.registers[11] as i64,
                        self.registers[12] as i64,
                        self.registers[13] as i64,
                        self.registers[14] as i64,
                    )
                } as u64;
            }
        }
    }
}

impl Index<RegisterName> for Cpu {
    type Output = u64;

    fn index(&self, index: RegisterName) -> &Self::Output {
        usize::from(index)
            .checked_sub(1)
            .and_then(|i| self.registers.get(i))
            .unwrap_or(&0)
    }
}

impl IndexMut<RegisterName> for Cpu {
    fn index_mut(&mut self, index: RegisterName) -> &mut Self::Output {
        usize::from(index)
            .checked_sub(1)
            .and_then(|i| self.registers.get_mut(i))
            .unwrap_or(&mut self.zero)
    }
}

/// Interprets the low 12 bits of the operand as a signed integer and
/// sign-extends it to fill 32 bits again.
const fn sign_extend_12bit(imm: u32) -> i32 {
    u32_sms(imm, 0, 12, 20) as i32 >> 20
}
