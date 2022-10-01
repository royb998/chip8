// ----- Modules ----- //

pub mod instructions;

// ----- Imports ----- //

// use std::thread::sleep;
// use std::time::Duration;

use crate::cpu::instructions::Instruction;
use crate::display::{Display, Sprite};
use crate::memory::address::Address;
use crate::memory::Memory;
use crate::registers::{PC, Registers};

// ----- Consts ----- //

// const INSTRUCTION_PAUSE: Duration = Duration::from_micros(1400);

// ----- Structs ----- //

pub struct CPU {
    display: Display,
    memory: Memory,
    registers: Registers,
    pc: PC,
}

impl CPU {
    pub fn new(exe_path: &str) -> CPU {
        CPU {
            display: Display::new(),
            memory: Memory::new(exe_path), // TODO: Load initial memory.
            registers: Registers::new(),
            pc: PC::new(),
        }
    }

    /// Fetch the next opcode from the memory.
    fn fetch(&mut self) -> u16 {
        let cur = self.pc.get();
        let data = self.memory.read(cur, 2);
        assert_eq!(data.len(), 2);

        self.pc.increment();
        return ((data[0] as u16) << 8) | (data[1] as u16);
    }

    /// Execute the given instruction on the CPU.
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::CLS() => { self.display.clear(); }
            Instruction::JUMP(addr) => { self.pc.set(addr); }
            Instruction::SETI(x, imm) => {
                self.registers.set_variable(x, imm);
            }
            Instruction::ADDI(x, imm) => {
                let current = self.registers.get_variable(x);
                let new = current + imm;
                self.registers.set_variable(x, new);
            }
            Instruction::SETN(imm) => {
                let addr = Address::from(imm as usize);
                self.registers.set_index(addr);
            }
            Instruction::DRAW(x, y, n) => { self.draw(x, y, n); }

            Instruction::ADDN(x) => {
                let index = self.registers.get_index();
                let x = self.registers.get_variable(x);
                let new = index.get() + x as usize;
                self.registers.set_index(Address::from(new));
            }
            _ => { assert!(false) }
        };
    } // TODO

    /// Perform one operation cycle (fetch-decode-execute).
    fn cycle(&mut self) {
        let opcode = self.fetch();
        let instruction = Instruction::from(opcode);
        // TODO: Invalid instruction.
        self.execute(instruction);
    }

    /// Run the cpu forever, I guess.
    pub fn run(&mut self) {
        loop {
            self.cycle();
            // sleep(INSTRUCTION_PAUSE);
        }
    }

    fn draw(&mut self, x_reg: usize, y_reg: usize, height: u8) {
        assert!(height <= 15);
        let addr = self.registers.get_index();
        let sprite_data = self.memory.read(addr, height as usize);
        let sprite = Sprite::from(sprite_data);
        let x = self.registers.get_variable(x_reg);
        let y = self.registers.get_variable(y_reg);

        self.registers.set_flag(false);
        let overflow = self.display.add_sprite(&sprite, x as usize, y as usize);
        self.registers.set_flag(overflow);
        self.display.show();
    }
}
