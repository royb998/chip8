// ----- Imports ----- //

use crate::Instruction::{*};
use crate::memory::address::Address;

// ----- Types ----- //

type Reg = u8;
type Imm = u8;
type Imm12 = u16;

// ----- Structs ----- //

#[derive(Debug)]
pub enum Instruction {
    CLS(),
    RET(),
    JUMP(Address),
    CALL(Address),
    // Skip instructions:
    SEQ(Reg, Imm),
    SNE(Reg, Imm),
    SRE(Reg, Reg),
    SRNE(Reg, Reg),

    SETM(Reg, Imm),
    ADDI(Reg, Imm),
    SETI(Imm12),

    INVALID(),
}

impl Instruction {
    pub fn from(opcode: u16) -> Instruction {
        // Get all possible values from opcode.
        let inst_group = ((opcode >> 12) & 0x0F);
        let imm: Imm = (opcode & 0xFF) as u8;
        let imm12: Imm12 = opcode & 0x0FFF;
        let address = Address::from(imm12);
        let x: Reg = ((opcode >> 8) & 0x0F) as u8;
        let y: Reg = ((opcode >> 4) & 0x0F) as u8;

        match inst_group {
            0 => {
                match opcode {
                    0x00E0 => { CLS() }
                    0x00EE => { RET() }
                    _ => { INVALID() }
                }
            }
            0x1 => { JUMP(address) }
            0x2 => { CALL(address) }
            0x3 => { SEQ(x, imm) }
            0x4 => { SNE(x, imm) }
            0x5 => { SRE(x, y) }
            0x6 => { SETM(x, imm) }
            0x7 => { ADDI(x, imm) }
            // 0x8 => {} // TODO: Arithmetic instructions
            0x9 => { SRNE(x, y) }
            0xa => { SETI(imm12) }
            // 0xb => {} // TODO: Jump offset
            // 0xc => {} // TODO: Random
            // 0xd => {} // TODO: Draw
            // 0xe => {} // TODO: Skip if Key
            // 0xf => {} // TODO: Others
            _ => { INVALID() }
        }
    }
}
