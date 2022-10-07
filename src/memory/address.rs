// ----- Consts ----- //

use std::fmt::Formatter;

pub const MAX_ADDRESS: usize = 0x0FFF;

// ----- Structs ----- //

pub struct Address {
    value: usize,
}

impl Address {
    pub fn get(&self) -> usize {
        return self.value;
    }
}

impl From<usize> for Address {
    /// Get address from the given `addr` (as in, u16). Dismiss the upper nibble
    /// and keep the 12-bit value as a valid address.
    fn from(addr: usize) -> Self {
        return Address { value: (addr & 0x0FFF) as usize };
    }
}

impl Clone for Address {
    fn clone(&self) -> Self {
        Address::from(self.get())
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address:0x{:03x}", self.value)
    }
}
