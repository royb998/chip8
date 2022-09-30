// ----- Consts ----- //

pub const MAX_ADDRESS: usize = 0x0FFF;

// ----- Structs ----- //

#[derive(Debug)]
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
