// ----- Modules ----- //

pub mod instructions;

// ----- Imports ----- //

// use std::thread::sleep;
// use std::time::Duration;
use rand::Rng;

use crate::cpu::instructions::Instruction;
use crate::display::{Display, Sprite};
use crate::memory::address::Address;
use crate::memory::Memory;
use crate::registers::{PC, Registers};
use crate::stack::Stack;
use crate::timers::Timer;

// ----- Consts ----- //

// const INSTRUCTION_PAUSE: Duration = Duration::from_micros(1400);

// ----- Structs ----- //

pub struct CPU {
    display: Display,
    memory: Memory,
    stack: Stack,
    registers: Registers,
    pc: PC,
    delay_timer: Timer,
    sound_timer: Timer,
}

impl CPU {
    pub fn new(exe_path: &str) -> CPU {
        CPU {
            display: Display::new(),
            memory: Memory::new(exe_path),
            stack: Stack::new(),
            registers: Registers::new(),
            pc: PC::new(),
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
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
            Instruction::RET() => {
                self.pc.set(self.stack.pop());
            }
            Instruction::JUMP(addr) => { self.pc.set(addr); }
            Instruction::CALL(addr) => {
                self.stack.push(self.pc.get());
                self.pc.set(addr);
            }
            Instruction::SEQ(x, imm) => {
                let value = self.registers.get_variable(x);
                if imm == value {
                    self.pc.increment();
                }
            }
            Instruction::SNE(x, imm) => {
                let value = self.registers.get_variable(x);
                if imm != value {
                    self.pc.increment();
                }
            }
            Instruction::SRE(x, y) => {
                let a = self.registers.get_variable(x);
                let b = self.registers.get_variable(y);
                if a == b {
                    self.pc.increment();
                }
            }
            Instruction::SRNE(x, y) => {
                let a = self.registers.get_variable(x);
                let b = self.registers.get_variable(y);
                if a != b {
                    self.pc.increment();
                }
            }
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
            Instruction::JMPO(addr) => {
                let offset = self.registers.get_variable(0) as usize;
                let target = addr.get();
                let new = Address::from(target + offset);
                self.pc.set(new);
            }
            Instruction::RAND(x, imm) => {
                let value = rand::thread_rng().gen_range(0..=0xFF);
                self.registers.set_variable(x, value & imm);
            }
            Instruction::DRAW(x, y, n) => { self.draw(x, y, n); }

            Instruction::ADDN(x) => {
                let index = self.registers.get_index();
                let x = self.registers.get_variable(x);
                let new = index.get() + x as usize;
                self.registers.set_index(Address::from(new));
            }
            Instruction::STD(x) => {
                let value = self.registers.get_variable(x);
                self.delay_timer.set(value);
            }
            Instruction::RDD(x) => {
                let value = self.delay_timer.get();
                self.registers.set_variable(x, value);
            }
            Instruction::STS(x) => {
                let value = self.registers.get_variable(x);
                self.sound_timer.set(value);
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
