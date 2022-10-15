// ----- Imports ----- //

use std::fmt::Formatter;
use crate::cpu::instructions::Instruction::{*};
use crate::memory::address::Address;

// ----- Structs ----- //

pub enum Instruction {
    CLS(),

    // Flow control:
    RET(),
    JUMP { address: Address },
    CALL { address: Address },
    JMPO { address: Address },
    SETN { address: Address },
    ADDN { reg: usize },

    // Skip instructions:
    SEQ { reg: usize, imm8: u8 },
    SNE { reg: usize, imm8: u8 },
    SRE { reg_x: usize, reg_y: usize },
    SRNE { reg_x: usize, reg_y: usize },

    // Arithmetics:
    SETI { reg: usize, imm8: u8 },
    ADDI { reg: usize, imm8: u8 },
    SET { reg_x: usize, reg_y: usize },
    OR { reg_x: usize, reg_y: usize },
    AND { reg_x: usize, reg_y: usize },
    XOR { reg_x: usize, reg_y: usize },
    ADD { reg_x: usize, reg_y: usize },
    SUB { reg_x: usize, reg_y: usize },
    NSUB { reg_x: usize, reg_y: usize },
    SHL { reg_x: usize, reg_y: usize },
    SHR { reg_x: usize, reg_y: usize },

    // Timers
    STD { reg: usize },
    RDD { reg: usize },
    STS { reg: usize },

    RAND { reg: usize, imm8: u8 },
    DRAW { reg_x: usize, reg_y: usize, imm4: u8 },

    // Key input
    SKE { reg: usize },
    SKN { reg: usize },
    GTK { reg: usize },

    FONT { reg: usize },
    BCD { reg: usize },
    STM { reg: usize },
    LDM { reg: usize },

    INVALID { opcode: u16 },
}

impl From<u16> for Instruction {
    fn from(opcode: u16) -> Self {
        // Get all possible values from opcode.
        let inst_group = (opcode >> 12) & 0x0F;
        let imm4: u8 = (opcode & 0x0F) as u8;
        let imm8: u8 = (opcode & 0xFF) as u8;
        let imm12: u16 = opcode & 0x0FFF;
        let address = Address::from(imm12 as usize);
        let reg_x: usize = ((opcode >> 8) & 0x0F) as usize;
        let reg_y: usize = ((opcode >> 4) & 0x0F) as usize;

        match inst_group {
            0x0 => {
                match imm12 {
                    0x0E0 => { CLS() }
                    0x0EE => { RET() }
                    _ => { INVALID { opcode } }
                }
            }
            0x1 => { JUMP { address } }
            0x2 => { CALL { address } }
            0x3 => { SEQ { reg: reg_x, imm8 } }
            0x4 => { SNE { reg: reg_x, imm8 } }
            0x5 => { SRE { reg_x, reg_y } }
            0x6 => { SETI { reg: reg_x, imm8 } }
            0x7 => { ADDI { reg: reg_x, imm8 } }
            0x8 => {
                match imm4 {
                    0x0 => { SET { reg_x, reg_y } }
                    0x1 => { OR { reg_x, reg_y } }
                    0x2 => { AND { reg_x, reg_y } }
                    0x3 => { XOR { reg_x, reg_y } }
                    0x4 => { ADD { reg_x, reg_y } }
                    0x5 => { SUB { reg_x, reg_y } }
                    0x7 => { NSUB { reg_x, reg_y } }
                    0x6 => { SHR { reg_x, reg_y } }
                    0xe => { SHL { reg_x, reg_y } }
                    _ => { INVALID { opcode } }
                }
            }
            0x9 => { SRNE { reg_x, reg_y } }
            0xa => { SETN { address } }
            0xb => { JMPO { address } }
            0xc => { RAND { reg: reg_x, imm8 } }
            0xd => { DRAW { reg_x, reg_y, imm4 } }
            0xe => {
                match imm8 {
                    0x9e => { SKE { reg: reg_x } }
                    0xa1 => { SKN { reg: reg_x } }
                    _ => { INVALID { opcode } }
                }
            }
            0xf => {
                match imm8 {
                    0x1e => { ADDN { reg: reg_x } }
                    0x07 => { RDD { reg: reg_x } }
                    0x0a => { GTK { reg: reg_x } }
                    0x15 => { STD { reg: reg_x } }
                    0x18 => { STS { reg: reg_x } }
                    0x29 => { FONT { reg: reg_x } }
                    0x33 => { BCD { reg: reg_x } }
                    0x55 => { STM { reg: reg_x } }
                    0x65 => { LDM { reg: reg_x } }
                    _ => { INVALID { opcode } }
                }
            }
            _ => { INVALID { opcode } }
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CLS() => { write!(f, "CLS()") }
            RET() => { write!(f, "RET()") }
            JUMP { address } => { write!(f, "JUMP({:03x})", address.get()) }
            CALL { address } => { write!(f, "CALL({:03x})", address.get()) }
            SEQ { reg, imm8 } => { write!(f, "SEQ(v{:x}, {:02x})", reg, imm8) }
            SNE { reg, imm8 } => { write!(f, "SNE(v{:x}, {:02x})", reg, imm8) }
            SRE { reg_x, reg_y } => { write!(f, "SRE(v{:x}, v{:x})", reg_x, reg_y) }
            SRNE { reg_x, reg_y } => { write!(f, "SRNE(v{:x}, v{:x})", reg_x, reg_y) }
            SETI { reg, imm8 } => { write!(f, "SETI(v{:x}, {:02x})", reg, imm8) }
            ADDI { reg, imm8 } => { write!(f, "ADDI(v{:x}, {:02x})", reg, imm8) }
            SETN { address } => { write!(f, "SETN({:03x})", address.get()) }
            SET { reg_x, reg_y } => { write!(f, "SET(v{:x}, v{:x})", reg_x, reg_y) }
            OR { reg_x, reg_y } => { write!(f, "OR(v{:x}, v{:x})", reg_x, reg_y) }
            AND { reg_x, reg_y } => { write!(f, "AND(v{:x}, v{:x})", reg_x, reg_y) }
            XOR { reg_x, reg_y } => { write!(f, "XOR(v{:x}, v{:x})", reg_x, reg_y) }
            ADD { reg_x, reg_y } => { write!(f, "ADD(v{:x}, v{:x})", reg_x, reg_y) }
            SUB { reg_x, reg_y } => { write!(f, "SUB(v{:x}, v{:x})", reg_x, reg_y) }
            NSUB { reg_x, reg_y } => { write!(f, "NSUB(v{:x}, v{:x})", reg_x, reg_y) }
            SHR { reg_x, reg_y } => { write!(f, "SHR(v{:x}, v{:x})", reg_x, reg_y) }
            SHL { reg_x, reg_y } => { write!(f, "SHL(v{:x}, v{:x})", reg_x, reg_y) }
            JMPO { address } => { write!(f, "JMPO({:03x})", address.get()) }
            RAND { reg, imm8 } => { write!(f, "RAND(v{:x}, {:02x})", reg, imm8) }
            DRAW { reg_x, reg_y, imm4 } => { write!(f, "DRAW(v{:x}, v{:x}, {:x})", reg_x, reg_y, imm4) }
            ADDN { reg } => { write!(f, "ADDN(v{:x})", reg) }
            RDD { reg } => { write!(f, "RDD(v{:x})", reg) }
            STD { reg } => { write!(f, "STD(v{:x})", reg) }
            STS { reg } => { write!(f, "STS(v{:x})", reg) }
            FONT { reg } => { write!(f, "FONT(v{:x})", reg) }
            BCD { reg } => { write!(f, "BCD(v{:x})", reg) }
            STM { reg } => { write!(f, "STM(v{:x})", reg) }
            LDM { reg } => { write!(f, "LDM(v{:x})", reg) }
            SKE { reg } => { write!(f, "SKE(v{:x})", reg) }
            SKN { reg } => { write!(f, "SKN(v{:x})", reg) }
            GTK { reg } => { write!(f, "GTK(v{:x})", reg) }
            INVALID { opcode } => { write!(f, "INVALID({:04x})", opcode) }
        }
    }
}
