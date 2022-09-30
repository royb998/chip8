// ----- Modules ----- //

pub mod address;

// ----- Imports ----- //

use address::Address;

// ----- Consts ----- //

const MEMORY_SIZE: usize = 0x1000;  // 2 ** 12

// ----- Structs ----- //

pub struct Memory {
    buffer: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        let result = Memory {
            buffer: [0; MEMORY_SIZE]
        };

        return result;
    }

    /// Read `len` bytes from memory, starting at the given `address`. If too
    /// much data is requested (i.e. last address is oob), read as much as
    /// possible and stop there.
    ///
    /// returns Vector with read data.
    pub fn read(&self, address: Address, len: usize) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let index = address.get();

        for i in 0..len {
            if index + i > MEMORY_SIZE {
                break;
            }
            result.push(self.buffer[index + i]);
        }

        return result;
    }

    /// Write the given `data` to memory, at the given `address`. If too much
    /// data is given (i.e. last address is oob), write as much as possible and
    /// stop there.
    ///
    /// returns Amount of bytes read.
    pub fn write(&mut self, address: Address, data: &Vec<u8>) -> usize {
        let mut len: usize = 0;
        let index = address.get();

        // TODO: Raise error upon invalid address?

        for (i, byte) in data.iter().enumerate() {
            if index + i > MEMORY_SIZE {
                break;
            }

            self.buffer[index + i] = *byte;
            len += 1;
        }

        return len;
    }
}
