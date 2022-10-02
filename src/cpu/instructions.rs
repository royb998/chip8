// ----- Imports ----- //

use crate::cpu::instructions::Instruction::{*};
use crate::memory::address::Address;

// ----- Types ----- //

type Reg = usize;
type Imm4 = u8;
type Imm8 = u8;
type Imm12 = u16;

// ----- Structs ----- //

#[derive(Debug)]
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
    SHL(Reg, Reg),
    SHR(Reg, Reg),

    // Timers
    STD(Reg),
    RDD(Reg),
    STS(Reg),

    RAND(Reg, Imm8),
    DRAW(Reg, Reg, Imm4),
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
                match opcode {
                    0x00E0 => { CLS() }
                    0x00EE => { RET() }
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
                    0x7 => { SUB(y, x) }
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
            // 0xe => {} // TODO: Skip if Key
            0xf => {
                match imm8 {
                    0x1e => { ADDN(x) }
                    0x07 => { RDD(x) }
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
