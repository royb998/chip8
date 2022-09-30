// ----- Modules ----- //

pub mod instructions;

// ----- Imports ----- //

use crate::cpu::instructions::Instruction;
use crate::display::Display;
use crate::memory::Memory;
use crate::registers::Registers;

// ----- Structs ----- //

pub struct CPU {
    display: Display,
    memory: Memory,
    registers: Registers,
}
