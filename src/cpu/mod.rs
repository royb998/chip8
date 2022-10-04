// ----- Modules ----- //

pub mod instructions;

// ----- Imports ----- //

use rand::Rng;

use crate::cpu::instructions::Instruction;
use crate::display::{Display, Sprite};
use crate::{keyboard, memory};
use crate::memory::address::Address;
use crate::memory::Memory;
use crate::registers::{PC, Registers};
use crate::stack::Stack;
use crate::timers::Timer;

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
                let current = self.registers.get_variable(x) as u16;
                let new = (current + imm as u16) as u8;
                self.registers.set_variable(x, new);
            }
            Instruction::SETN(addr) => {
                self.registers.set_index(addr);
            }
            Instruction::SET(x, y) => {
                let value = self.registers.get_variable(y);
                self.registers.set_variable(x, value);
            }
            Instruction::OR(x, y) => {
                let a = self.registers.get_variable(x);
                let b = self.registers.get_variable(y);
                self.registers.set_variable(x, a | b);
            }
            Instruction::AND(x, y) => {
                let a = self.registers.get_variable(x);
                let b = self.registers.get_variable(y);
                self.registers.set_variable(x, a & b);
            }
            Instruction::XOR(x, y) => {
                let a = self.registers.get_variable(x);
                let b = self.registers.get_variable(y);
                self.registers.set_variable(x, a ^ b);
            }
            Instruction::ADD(x, y) => {
                let a = self.registers.get_variable(x) as u16;
                let b = self.registers.get_variable(y) as u16;
                let result = a + b;

                self.registers.set_flag(result > 0xFF);
                self.registers.set_variable(x, result as u8);
            }
            Instruction::SUB(x, y) => {
                let a = 0x0100 + self.registers.get_variable(x) as u16;
                let b = self.registers.get_variable(y) as u16;
                let result = a - b;

                self.registers.set_flag(result & 0x0100 > 0);
                self.registers.set_variable(x, result as u8);
            }
            Instruction::NSUB(x, y) => {
                let a = 0x0100 + self.registers.get_variable(y) as u16;
                let b = self.registers.get_variable(x) as u16;
                let result = a - b;

                self.registers.set_flag(result & 0x0100 > 0);
                self.registers.set_variable(x, result as u8);
            }
            Instruction::SHR(x, y) => {
                let value = self.registers.get_variable(y);

                self.registers.set_flag((value & 0x01) > 0);
                self.registers.set_variable(x, value >> 1);
            }
            Instruction::SHL(x, y) => {
                let value = self.registers.get_variable(y);

                self.registers.set_flag((value & 0x80) > 0);
                self.registers.set_variable(x, value << 1);
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
            Instruction::SKE(x) => {
                let value = self.registers.get_variable(x) & 0x0F;
                let key = keyboard::Key::from(value);

                if let Some(pressed) = keyboard::get_key() {
                    if pressed == key {
                        self.pc.increment();
                    }
                }
            }
            Instruction::SKN(x) => {
                let value = self.registers.get_variable(x) & 0x0F;
                let key = keyboard::Key::from(value);

                if let Some(pressed) = keyboard::get_key() {
                    if pressed == key {
                        return;
                    }
                }
                self.pc.increment();
            }
            Instruction::GTK(x) => {
                if let Some(key) = keyboard::get_key() {
                    let value = key.get();
                    if value != keyboard::INVALID_KEY {
                        self.registers.set_variable(x, key.get());
                        return;
                    }
                }
                self.pc.decrement();
            }
            Instruction::ADDN(x) => {
                let index = self.registers.get_index();
                let x = self.registers.get_variable(x);
                let new = index.get() + x as usize;
                self.registers.set_index(Address::from(new));
            }
            Instruction::RDD(x) => {
                let value = self.delay_timer.get();
                self.registers.set_variable(x, value);
            }
            Instruction::STD(x) => {
                let value = self.registers.get_variable(x);
                self.delay_timer.set(value);
            }
            Instruction::STS(x) => {
                let value = self.registers.get_variable(x);
                self.sound_timer.set(value);
            }
            Instruction::FONT(x) => {
                let mut i = memory::FONT_ADDR;
                let hex = self.registers.get_variable(x) & 0x0F;
                i += memory::FONT_HEIGHT * (hex as usize);
                self.registers.set_index(Address::from(i));
            }
            Instruction::BCD(x) => {
                let mut value = self.registers.get_variable(x);
                let mut digits: Vec<u8> = Vec::new();

                while value > 0 {
                    let current = value % 10;
                    digits.insert(0, current);
                    value /= 10;
                }

                let address = self.registers.get_index();
                self.memory.write(address, &digits);
            }
            Instruction::STM(x) => {
                let mut data: Vec<u8> = Vec::new();

                for i in 0..=x {
                    data.push(self.registers.get_variable(i));
                }

                let address = self.registers.get_index();
                // let new = Address::from(address.get() + x as usize + 1);
                // self.registers.set_index(new);

                self.memory.write(address, &data);
            }
            Instruction::LDM(x) => {
                let address = self.registers.get_index();
                // let new = Address::from(address.get() + x as usize + 1);
                // self.registers.set_index(new);

                let data = self.memory.read(address, x + 1);
                assert_eq!(data.len(), x + 1);

                for (i, value) in data.iter().enumerate() {
                    self.registers.set_variable(i, *value);
                }
            }
            _ => { panic!("Tried to run instruction at {}; found {}", self.pc.get(), instruction); }
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
    }
}
