use crate::{
    bits::SignExtend,
    error::Result,
    instruction::{
        BFunct, IFunct, Instruction, NeedMoreBytes, RFunct, RegisterName,
        SFunct, UOpcode,
    },
    Opts,
};
use std::ops::{Index, IndexMut};

const STACK_SIZE: usize = 4096;

pub struct Cpu {
    opts: Opts,
    zero: u64, // Never read from this
    registers: [u64; 31],
    pc: *const u16,
    old_pc: *const u16,
    #[allow(dead_code)]
    stack: Vec<u8>,
}

impl Cpu {
    pub fn new(opts: Opts, pc: *const u16) -> Self {
        let stack = Vec::<u8>::with_capacity(STACK_SIZE);
        let mut registers: [u64; 31] = Default::default();
        registers[1] = stack.as_ptr().wrapping_add(STACK_SIZE) as u64;
        Self {
            opts,
            zero: 0,
            registers,
            pc,
            old_pc: pc,
            stack,
        }
    }

    pub unsafe fn run(&mut self) -> Result<()> {
        loop {
            unsafe { self.step() }?;
        }
    }

    unsafe fn step(&mut self) -> Result<()> {
        self.old_pc = self.pc;
        let low_half = unsafe { *self.pc };
        let instruction = match Instruction::try_from(low_half) {
            Ok(instruction) => {
                self.pc = self.pc.wrapping_add(1);
                instruction
            }
            Err(Ok(NeedMoreBytes)) => {
                let high_half = unsafe { *self.pc.wrapping_add(1) };
                let raw_instruction =
                    u32::from(high_half) << 16 | u32::from(low_half);
                self.pc = self.pc.wrapping_add(2);
                Instruction::try_from(raw_instruction)?
            }
            Err(Err(err)) => return Err(err),
        };
        unsafe {
            self.run_instruction(instruction);
        }
        Ok(())
    }

    unsafe fn run_instruction(&mut self, instruction: Instruction) {
        if self.opts.verbose {
            eprintln!("Running: {instruction:?}");
        }
        match instruction {
            Instruction::R {
                funct,
                rs2,
                rs1,
                rd,
            } => {
                let rs1 = self[rs1];
                let rs2 = self[rs2];
                self[rd] = match funct {
                    RFunct::Add => (rs1 as i64).wrapping_add(rs2 as i64) as u64,
                    RFunct::Sub => (rs1 as i64).wrapping_sub(rs2 as i64) as u64,
                    RFunct::Sll => rs1.wrapping_shl(rs2 as u32),
                    RFunct::Slt => u64::from((rs1 as i64) < rs2 as i64),
                    RFunct::Sltu => u64::from(rs1 < rs2),
                    RFunct::Xor => rs1 ^ rs2,
                    RFunct::Srl => rs1.wrapping_shr(rs2 as u32),
                    RFunct::Sra => (rs1 as i64).wrapping_shr(rs2 as u32) as u64,
                    RFunct::Or => rs1 | rs2,
                    RFunct::And => rs1 & rs2,
                }
            }
            Instruction::I {
                imm,
                rs1,
                funct,
                rd,
            } => {
                let imm_i32 = sign_extend_12bit(imm);
                let rs1 = self[rs1];
                match funct {
                    IFunct::Addi => {
                        self[rd] = rs1.wrapping_add_signed(imm_i32.into());
                    }
                    IFunct::Slti => {
                        self[rd] = u64::from((rs1 as i64) < i64::from(imm_i32));
                    }
                    IFunct::Sltiu => {
                        self[rd] = u64::from(rs1 < imm_i32.sign_extend());
                    }
                    IFunct::Xori => self[rd] = rs1 ^ imm_i32.sign_extend(),
                    IFunct::Ori => self[rd] = rs1 | imm_i32.sign_extend(),
                    IFunct::Andi => self[rd] = rs1 & imm_i32.sign_extend(),
                    IFunct::Slli => todo!(),
                    IFunct::Srli => todo!(),
                    IFunct::Srai => todo!(),
                    IFunct::Lb => {
                        self[rd] = i64::from(unsafe {
                            *(rs1.wrapping_add_signed(i64::from(imm_i32))
                                as *const i8)
                        }) as u64;
                    }
                    IFunct::Lh => {
                        self[rd] = i64::from(unsafe {
                            *(rs1.wrapping_add_signed(i64::from(imm_i32))
                                as *const i16)
                        }) as u64;
                    }
                    IFunct::Lw => {
                        self[rd] = i64::from(unsafe {
                            *(rs1.wrapping_add_signed(i64::from(imm_i32))
                                as *const i32)
                        }) as u64;
                    }
                    IFunct::Ld => {
                        self[rd] = unsafe {
                            *(rs1.wrapping_add_signed(i64::from(imm_i32))
                                as *const u64)
                        };
                    }
                    IFunct::Lbu => {
                        self[rd] = u64::from(unsafe {
                            *(rs1.wrapping_add_signed(i64::from(imm_i32))
                                as *const u8)
                        });
                    }
                    IFunct::Lhu => {
                        self[rd] = u64::from(unsafe {
                            *(rs1.wrapping_add_signed(i64::from(imm_i32))
                                as *const u16)
                        });
                    }
                    IFunct::Jalr => {
                        self[rd] = self.pc as u64;
                        self.pc = rs1.wrapping_add_signed(i64::from(imm_i32))
                            as *const u16;
                    }
                }
            }
            Instruction::S {
                imm,
                rs2,
                rs1,
                funct,
            } => {
                let dest = self[rs1].wrapping_add_signed(i64::from(
                    sign_extend_12bit(u32::from(imm)),
                ));
                match funct {
                    SFunct::Sb => unsafe {
                        *(dest as *mut u8) = self[rs2] as _;
                    },
                    SFunct::Sh => unsafe {
                        *(dest as *mut u16) = self[rs2] as _;
                    },
                    SFunct::Sw => unsafe {
                        *(dest as *mut u32) = self[rs2] as _;
                    },
                    SFunct::Sd => unsafe {
                        *(dest as *mut u64) = self[rs2];
                    },
                }
            }
            Instruction::B {
                imm,
                rs2,
                rs1,
                funct,
            } => {
                let rs1 = self[rs1];
                let rs2 = self[rs2];
                let branch_condition = match funct {
                    BFunct::Beq => rs1 == rs2,
                    BFunct::Bne => rs1 != rs2,
                    BFunct::Blt => (rs1 as i64) < rs2 as i64,
                    BFunct::Bge => rs1 as i64 >= rs2 as i64,
                    BFunct::Bltu => rs1 < rs2,
                    BFunct::Bgeu => rs1 >= rs2,
                };
                if branch_condition {
                    self.pc = self
                        .old_pc
                        .cast::<u8>() // For single byte offsets
                        .wrapping_offset(isize::from(imm))
                        .cast();
                }
            }
            Instruction::U { imm, rd, opcode } => match opcode {
                UOpcode::Lui => self[rd] = imm.sign_extend(),
                UOpcode::Auipc => {
                    self[rd] = (self.old_pc as u64)
                        .wrapping_add_signed(i64::from(imm));
                }
            },
            Instruction::Jal { imm, rd } => {
                self[rd] = self.pc as u64;
                self.pc = self
                    .old_pc
                    .cast::<u8>() // For single byte offsets
                    .wrapping_offset(imm as isize)
                    .cast();
            }
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
    (imm << 20) as i32 >> 20
}
