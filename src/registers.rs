// ----- Consts ----- //

const VARIABLE_COUNT: usize = 0x10;
const FLAG_INDEX: usize = 0xF;

// ----- Structs ----- //

pub struct Registers {
    i: u16,  // Index register
    v: [u8; VARIABLE_COUNT],  // Variable registers
}

/// Program counter register.
pub struct PC {
    value: u16,
}

impl Registers {
    pub fn new() -> Registers {
        return Registers {
            i: 0,
            v: [0; VARIABLE_COUNT],
        };
    }

    pub fn get_variable(&self, index: usize) -> u8 {
        // TODO: Raise error in case of invalid index.
        if index >= VARIABLE_COUNT {
            return 0xFF;
        }

        return self.v[index];
    }

    pub fn set_variable(&mut self, index: usize, value: u8) {
        // TODO: Raise error in case of invalid index.
        if index >= VARIABLE_COUNT {
            return;
        }

        self.v[index] = value;
    }

    pub fn get_index(&self) -> u16 {
        return self.i;
    }

    pub fn set_index(&mut self, value: u16) {
        self.i = value;
    }

    pub fn get_flag(&self) -> bool {
        return self.v[FLAG_INDEX] > 0;
    }

    pub fn set_flag(&mut self) {
        self.v[FLAG_INDEX] = 1;
    }

    pub fn reset_flag(&mut self) {
        self.v[FLAG_INDEX] = 0;
    }
}

impl PC {
    pub fn new() -> PC {
        return PC {
            value: 0,
        };
    }

    pub fn get(&self) -> u16 {
        return self.value;
    }

    pub fn set(&mut self, value: u16) {
        // TODO: Raise error if needed.
        if value > 0xFFF {
            return;
        }

        self.value = value;
    }

    pub fn increment(&mut self) {
        self.value += 2;
    }
}
