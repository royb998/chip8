// ----- Modules ----- //

pub mod instructions;

// ----- Imports ----- //

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use spin_sleep::sleep;
use rand::Rng;

use crate::cpu::instructions::Instruction;
use crate::display::{Display, Sprite};
use crate::{keyboard, memory};
use crate::memory::address::Address;
use crate::memory::Memory;
use crate::registers::{PC, Registers};
use crate::stack::Stack;
use crate::timers::Timer;

// ----- Consts ----- //

const INSTRUCTION_PAUSE: Duration = Duration::from_micros(1400);

// ----- Structs ----- //

pub struct CPU {
    display: Display,
    memory: Rc<RefCell<Memory>>,
    stack: Stack,
    registers: Registers,
    pc: PC,
    delay_timer: Timer,
    sound_timer: Timer,
}

impl CPU {
    pub fn new(exe_path: &str) -> Self {
        let memory = Rc::new(RefCell::new(Memory::new(exe_path)));

        CPU {
            display: Display::new(),
            memory: Rc::clone(&memory),
            stack: Stack::new(Rc::clone(&memory)),
            registers: Registers::new(),
            pc: PC::new(),
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
        }
    }

    /// Fetch the next opcode from the memory.
    fn fetch(&mut self) -> u16 {
        let cur = self.pc.get();
        let data = self.memory.borrow().read(cur, 2);
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
            Instruction::JUMP { address } => { self.pc.set(address); }
            Instruction::CALL { address } => {
                self.stack.push(self.pc.get());
                self.pc.set(address);
            }
            Instruction::SEQ { reg, imm8 } => {
                let value = self.registers.get_variable(reg);
                if imm8 == value {
                    self.pc.increment();
                }
            }
            Instruction::SNE { reg, imm8 } => {
                let value = self.registers.get_variable(reg);
                if imm8 != value {
                    self.pc.increment();
                }
            }
            Instruction::SRE { reg_x, reg_y } => {
                let a = self.registers.get_variable(reg_x);
                let b = self.registers.get_variable(reg_y);
                if a == b {
                    self.pc.increment();
                }
            }
            Instruction::SRNE { reg_x, reg_y } => {
                let a = self.registers.get_variable(reg_x);
                let b = self.registers.get_variable(reg_y);
                if a != b {
                    self.pc.increment();
                }
            }
            Instruction::SETI { reg, imm8 } => {
                self.registers.set_variable(reg, imm8);
            }
            Instruction::ADDI { reg, imm8 } => {
                let current = self.registers.get_variable(reg) as u16;
                let new = (current + imm8 as u16) as u8;
                self.registers.set_variable(reg, new);
            }
            Instruction::SETN { address } => {
                self.registers.set_index(address);
            }
            Instruction::SET { reg_x, reg_y } => {
                let value = self.registers.get_variable(reg_y);
                self.registers.set_variable(reg_x, value);
            }
            Instruction::OR { reg_x, reg_y } => {
                let a = self.registers.get_variable(reg_x);
                let b = self.registers.get_variable(reg_y);
                self.registers.set_variable(reg_x, a | b);
            }
            Instruction::AND { reg_x, reg_y } => {
                let a = self.registers.get_variable(reg_x);
                let b = self.registers.get_variable(reg_y);
                self.registers.set_variable(reg_x, a & b);
            }
            Instruction::XOR { reg_x, reg_y } => {
                let a = self.registers.get_variable(reg_x);
                let b = self.registers.get_variable(reg_y);
                self.registers.set_variable(reg_x, a ^ b);
            }
            Instruction::ADD { reg_x, reg_y } => {
                let a = self.registers.get_variable(reg_x) as u16;
                let b = self.registers.get_variable(reg_y) as u16;
                let result = a + b;

                self.registers.set_flag(result > 0xFF);
                self.registers.set_variable(reg_x, result as u8);
            }
            Instruction::SUB { reg_x, reg_y } => {
                let a = 0x0100 + self.registers.get_variable(reg_x) as u16;
                let b = self.registers.get_variable(reg_y) as u16;
                let result = a - b;

                self.registers.set_flag(result & 0x0100 > 0);
                self.registers.set_variable(reg_x, result as u8);
            }
            Instruction::NSUB { reg_x, reg_y } => {
                let a = 0x0100 + self.registers.get_variable(reg_y) as u16;
                let b = self.registers.get_variable(reg_x) as u16;
                let result = a - b;

                self.registers.set_flag(result & 0x0100 > 0);
                self.registers.set_variable(reg_x, result as u8);
            }
            Instruction::SHR { reg_x, reg_y } => {
                let value = self.registers.get_variable(reg_y);

                self.registers.set_flag((value & 0x01) > 0);
                self.registers.set_variable(reg_x, value >> 1);
            }
            Instruction::SHL { reg_x, reg_y } => {
                let value = self.registers.get_variable(reg_y);

                self.registers.set_flag((value & 0x80) > 0);
                self.registers.set_variable(reg_x, value << 1);
            }
            Instruction::JMPO { address } => {
                let offset = self.registers.get_variable(0) as usize;
                let target = address.get();
                let new = Address::from(target + offset);
                self.pc.set(new);
            }
            Instruction::RAND { reg, imm8 } => {
                let value = rand::thread_rng().gen_range(0..=0xFF);
                self.registers.set_variable(reg, value & imm8);
            }
            Instruction::DRAW { reg_x, reg_y, imm4 } => { self.draw(reg_x, reg_y, imm4); }
            Instruction::SKE { reg } => {
                let value = self.registers.get_variable(reg) & 0x0F;
                let key = keyboard::Key::from(value);

                if let Some(pressed) = keyboard::get_key() {
                    if pressed == key {
                        self.pc.increment();
                    }
                }
            }
            Instruction::SKN { reg } => {
                let value = self.registers.get_variable(reg) & 0x0F;
                let key = keyboard::Key::from(value);

                if let Some(pressed) = keyboard::get_key() {
                    if pressed == key {
                        return;
                    }
                }
                self.pc.increment();
            }
            Instruction::GTK { reg } => {
                if let Some(key) = keyboard::get_key() {
                    let value = key.get();
                    if value != keyboard::INVALID_KEY {
                        self.registers.set_variable(reg, key.get());
                        return;
                    }
                }
                self.pc.decrement();
            }
            Instruction::ADDN { reg } => {
                let index = self.registers.get_index();
                let reg = self.registers.get_variable(reg);
                let new = index.get() + reg as usize;
                self.registers.set_index(Address::from(new));
            }
            Instruction::RDD { reg } => {
                let value = self.delay_timer.get();
                self.registers.set_variable(reg, value);
            }
            Instruction::STD { reg } => {
                let value = self.registers.get_variable(reg);
                self.delay_timer.set(value);
            }
            Instruction::STS { reg } => {
                let value = self.registers.get_variable(reg);
                self.sound_timer.set(value);
            }
            Instruction::FONT { reg } => {
                let mut i = memory::FONT_ADDR;
                let hex = self.registers.get_variable(reg) & 0x0F;
                i += memory::FONT_HEIGHT * (hex as usize);
                self.registers.set_index(Address::from(i));
            }
            Instruction::BCD { reg } => {
                let value = self.registers.get_variable(reg);
                let digits = [
                    value / 100,
                    (value / 10) % 10,
                    value % 10,
                ];

                let address = self.registers.get_index();
                self.memory.borrow_mut().write(address, &digits);
            }
            Instruction::STM { reg } => {
                let mut data: Vec<u8> = Vec::new();

                for i in 0..=reg {
                    data.push(self.registers.get_variable(i));
                }

                let address = self.registers.get_index();
                // let new = Address::from(address.get() + reg as usize + 1);
                // self.registers.set_index(new);

                self.memory.borrow_mut().write(address, &data);
            }
            Instruction::LDM { reg } => {
                let address = self.registers.get_index();
                // let new = Address::from(address.get() + reg as usize + 1);
                // self.registers.set_index(new);

                let data = self.memory.borrow().read(address, reg + 1);
                assert_eq!(data.len(), reg + 1);

                for (i, value) in data.iter().enumerate() {
                    self.registers.set_variable(i, *value);
                }
            }
            _ => { panic!("Tried to run instruction at {}; found {}", self.pc.get(), instruction); }
        };
    }

    /// Perform one operation cycle (fetch-decode-execute).
    fn cycle(&mut self) {
        let opcode = self.fetch();
        let instruction = Instruction::from(opcode);
        self.execute(instruction);
    }

    /// Run the cpu forever, I guess.
    pub fn run(&mut self) {
        loop {
            self.cycle();
            sleep(INSTRUCTION_PAUSE);
        }
    }

    fn draw(&mut self, x_reg: usize, y_reg: usize, height: u8) {
        assert!(height <= 15);
        let addr = self.registers.get_index();
        let sprite_data = self.memory.borrow().read(addr, height as usize);
        let sprite = Sprite::from(sprite_data);
        let x = self.registers.get_variable(x_reg);
        let y = self.registers.get_variable(y_reg);

        self.registers.set_flag(false);
        let overflow = self.display.add_sprite(&sprite, x as usize, y as usize);
        self.registers.set_flag(overflow);
    }
}
