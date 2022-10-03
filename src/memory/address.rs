// ----- Consts ----- //

use std::fmt::Formatter;

pub const MAX_ADDRESS: usize = 0x0FFF;

// ----- Structs ----- //

pub struct Address {
    value: usize,
}

impl Address {
    /// Get address from the given `addr` (as in, u16). Dismiss the upper nibble
    /// and keep the 12-bit value as a valid address.
    pub fn from(addr: usize) -> Address {
        return Address { value: (addr & 0x0FFF) as usize };
    }

    pub fn clone(&self) -> Address {
        return Address::from(self.value);
    }

    pub fn get(&self) -> usize {
        return self.value;
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address:0x{:03x}", self.value)
    }
}
