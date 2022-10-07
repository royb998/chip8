// ----- Imports ----- //

use crate::memory::address::Address;
use crate::memory::Memory;

// ----- Consts ----- //

const STACK_BASE: usize = 0;
const STACK_SIZE: usize = 0x20;
const ADDRESS_SIZE: usize = 2;

// ----- Structs ----- //

pub struct Stack {
    size: usize,
}

impl Stack {
    pub fn new() -> Self {
        return Stack {
            size: 0,
        };
    }

    pub fn push(&mut self, address: Address, memory: &mut Memory) {
        if self.size >= STACK_SIZE {
            panic!("Stack overflow! Call stack is full.");
        }

        let write_addr = Address::from(STACK_BASE + (self.size * ADDRESS_SIZE));
        let address_value = address.get();
        let address_data = vec![
            (address_value & 0xFF) as u8,
            ((address_value >> 8) & 0xFF) as u8,
        ];
        memory.write(write_addr, &address_data);
        self.size += 1;
    }

    pub fn pop(&mut self, memory: &Memory) -> Address {
        if self.size == 0 {
            panic!("Call stack is empty.");
        }

        self.size -= 1;
        let read_address = Address::from(STACK_BASE + (self.size * STACK_SIZE));
        let address_data = memory.read(read_address, ADDRESS_SIZE);
        let output_value = (address_data[0] as usize) | ((address_data[1] as usize) << 8);

        return Address::from(output_value);
    }
}
