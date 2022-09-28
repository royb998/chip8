// ----- Imports ----- //

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
    pub fn read(&self, address: usize, len: usize) -> Vec<u8>
    {
        let mut result: Vec<u8> = Vec::new();

        // TODO: Raise error upon invalid address?

        for i in 0..len {
            if address + i > MEMORY_SIZE {
                break;
            }
            result.push(self.buffer[address + i]);
        }

        return result;
    }

    /// Write the given `data` to memory, at the given `address`. If too much
    /// data is given (i.e. last address is oob), write as much as possible and
    /// stop there.
    ///
    /// returns Amount of bytes read.
    pub fn write(&mut self, address: usize, data: &Vec<u8>) -> usize
    {
        let mut len: usize = 0;

        // TODO: Raise error upon invalid address?

        for (i, byte) in data.iter().enumerate() {
            if address + i > MEMORY_SIZE {
                break;
            }

            self.buffer[address + i] = *byte;
            len += 1;
        }

        return len;
    }
}
