// ----- Imports ----- //

use std::fmt::Formatter;
use crate::cpu::instructions::Instruction::{*};
use crate::memory::address::Address;

// ----- Types ----- //

type Reg = usize;
type Imm4 = u8;
type Imm8 = u8;
type Imm12 = u16;

// ----- Structs ----- //

pub enum Instruction {
    CLS(),

    // Flow control:
    RET(),
    JUMP(Address),
    CALL(Address),
    JMPO(Address),
    SETN(Address),
    ADDN(Reg),

    // Skip instructions:
    SEQ(Reg, Imm8),
    SNE(Reg, Imm8),
    SRE(Reg, Reg),
    SRNE(Reg, Reg),

    // Arithmetics:
    SETI(Reg, Imm8),
    ADDI(Reg, Imm8),
    SET(Reg, Reg),
    OR(Reg, Reg),
    AND(Reg, Reg),
    XOR(Reg, Reg),
    ADD(Reg, Reg),
    SUB(Reg, Reg),
    NSUB(Reg, Reg),
    SHL(Reg, Reg),
    SHR(Reg, Reg),

    // Timers
    STD(Reg),
    RDD(Reg),
    STS(Reg),

    RAND(Reg, Imm8),
    DRAW(Reg, Reg, Imm4),

    // Key input
    SKE(Reg),
    SKN(Reg),
    GTK(Reg),

    FONT(Reg),
    BCD(Reg),
    STM(Reg),
    LDM(Reg),

    INVALID(u16),
}

impl Instruction {
    pub fn from(opcode: u16) -> Instruction {
        // Get all possible values from opcode.
        let inst_group = (opcode >> 12) & 0x0F;
        let imm4: Imm4 = (opcode & 0x0F) as u8;
        let imm8: Imm8 = (opcode & 0xFF) as u8;
        let imm12: Imm12 = opcode & 0x0FFF;
        let address = Address::from(imm12 as usize);
        let x: Reg = ((opcode >> 8) & 0x0F) as usize;
        let y: Reg = ((opcode >> 4) & 0x0F) as usize;

        match inst_group {
            0x0 => {
                match imm12 {
                    0x0E0 => { CLS() }
                    0x0EE => { RET() }
                    _ => { INVALID(opcode) }
                }
            }
            0x1 => { JUMP(address) }
            0x2 => { CALL(address) }
            0x3 => { SEQ(x, imm8) }
            0x4 => { SNE(x, imm8) }
            0x5 => { SRE(x, y) }
            0x6 => { SETI(x, imm8) }
            0x7 => { ADDI(x, imm8) }
            0x8 => {
                match imm4 {
                    0x0 => { SET(x, y) }
                    0x1 => { OR(x, y) }
                    0x2 => { AND(x, y) }
                    0x3 => { XOR(x, y) }
                    0x4 => { ADD(x, y) }
                    0x5 => { SUB(x, y) }
                    0x7 => { NSUB(y, x) }
                    0x6 => { SHR(x, y) }
                    0xe => { SHL(x, y) }
                    _ => { INVALID(opcode) }
                }
            }
            0x9 => { SRNE(x, y) }
            0xa => { SETN(address) }
            0xb => { JMPO(address) }
            0xc => { RAND(x, imm8) }
            0xd => { DRAW(x, y, imm4) }
            0xe => {
                match imm8 {
                    0x9e => { SKE(x) }
                    0xa1 => { SKN(x) }
                    _ => { INVALID(opcode) }
                }
            }
            0xf => {
                match imm8 {
                    0x1e => { ADDN(x) }
                    0x07 => { RDD(x) }
                    0x0a => { GTK(x) }
                    0x15 => { STD(x) }
                    0x18 => { STS(x) }
                    0x29 => { FONT(x) }
                    0x33 => { BCD(x) }
                    0x55 => { STM(x) }
                    0x65 => { LDM(x) }
                    _ => { INVALID(opcode) }
                }
            }
            _ => { INVALID(opcode) }
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CLS() => { write!(f, "CLS()") }
            RET() => { write!(f, "RET()") }
            JUMP(addr) => { write!(f, "JUMP({:03x})", addr.get()) }
            CALL(addr) => { write!(f, "CALL({:03x})", addr.get()) }
            SEQ(x, imm) => { write!(f, "SEQ(v{:x}, {:02x})", x, imm) }
            SNE(x, imm) => { write!(f, "SNE(v{:x}, {:02x})", x, imm) }
            SRE(x, y) => { write!(f, "SRE(v{:x}, v{:x})", x, y) }
            SRNE(x, y) => { write!(f, "SRNE(v{:x}, v{:x})", x, y) }
            SETI(x, imm) => { write!(f, "SETI(v{:x}, {:02x})", x, imm) }
            ADDI(x, imm) => { write!(f, "ADDI(v{:x}, {:02x})", x, imm) }
            SETN(addr) => { write!(f, "SETN({:03x})", addr.get()) }
            SET(x, y) => { write!(f, "SET(v{:x}, v{:x})", x, y) }
            OR(x, y) => { write!(f, "OR(v{:x}, v{:x})", x, y) }
            AND(x, y) => { write!(f, "AND(v{:x}, v{:x})", x, y) }
            XOR(x, y) => { write!(f, "XOR(v{:x}, v{:x})", x, y) }
            ADD(x, y) => { write!(f, "ADD(v{:x}, v{:x})", x, y) }
            SUB(x, y) => { write!(f, "SUB(v{:x}, v{:x})", x, y) }
            NSUB(x, y) => { write!(f, "NSUB(v{:x}, v{:x})", x, y) }
            SHR(x, y) => { write!(f, "SHR(v{:x}, v{:x})", x, y) }
            SHL(x, y) => { write!(f, "SHL(v{:x}, v{:x})", x, y) }
            JMPO(addr) => { write!(f, "JMPO({:03x})", addr.get()) }
            RAND(x, imm) => { write!(f, "RAND(v{:x}, {:02x})", x, imm) }
            DRAW(x, y, n) => { write!(f, "DRAW(v{:x}, v{:x}, {:x})", x, y, n) }
            ADDN(x) => { write!(f, "ADDN(v{:x})", x) }
            RDD(x) => { write!(f, "RDD(v{:x})", x) }
            STD(x) => { write!(f, "STD(v{:x})", x) }
            STS(x) => { write!(f, "STS(v{:x})", x) }
            FONT(x) => { write!(f, "FONT(v{:x})", x) }
            BCD(x) => { write!(f, "BCD(v{:x})", x) }
            STM(x) => { write!(f, "STM(v{:x})", x) }
            LDM(x) => { write!(f, "LDM(v{:x})", x) }
            SKE(x) => { write!(f, "SKE(v{:x})", x) }
            SKN(x) => { write!(f, "SKN(v{:x})", x) }
            GTK(x) => { write!(f, "GTK(v{:x})", x) }
            INVALID(opcode) => { write!(f, "INVALID({:04x})", opcode) }
        }
    }
}
