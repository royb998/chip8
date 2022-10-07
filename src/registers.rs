// ----- Imports ----- //

use crate::memory::address::{MAX_ADDRESS, Address};

// ----- Consts ----- //

const VARIABLE_COUNT: usize = 0x10;
const FLAG_INDEX: usize = 0xF;
const INITIAL_PC: usize = 0x200;

// ----- Structs ----- //

pub struct Registers {
    i: Address,  // Index register
    v: [u8; VARIABLE_COUNT],  // Variable registers
}

/// Program counter register.
pub struct PC {
    value: Address,
}

impl Registers {
    pub fn new() -> Self {
        return Registers {
            i: Address::from(0),
            v: [0; VARIABLE_COUNT],
        };
    }

    pub fn get_variable(&self, index: usize) -> u8 {
        assert!(index < self.v.len());

        return self.v[index];
    }

    pub fn set_variable(&mut self, index: usize, value: u8) {
        assert!(index < self.v.len());

        self.v[index] = value;
    }

    pub fn get_index(&self) -> Address {
        return self.i;
    }

    pub fn set_index(&mut self, value: Address) {
        self.i = value;
    }

    pub fn set_flag(&mut self, value: bool) {
        self.v[FLAG_INDEX] = value as u8;
    }
}

impl PC {
    pub fn new() -> Self {
        return PC {
            value: Address::from(INITIAL_PC),
        };
    }

    pub fn get(&self) -> Address {
        return self.value.clone();
    }

    pub fn set(&mut self, value: Address) {
        self.value = value;
    }

    pub fn increment(&mut self) {
        let current = self.value.get();
        let new = current + 2;

        if new >= MAX_ADDRESS {
            panic!("Reached end of memory");
        }
        self.value = Address::from(new);
    }

    pub fn decrement(&mut self) {
        let current = self.value.get();
        let new = current - 2;

        if new >= MAX_ADDRESS {
            panic!("Reached end of memory");
        }
        self.value = Address::from(new);
    }
}
