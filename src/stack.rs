// ----- Imports ----- //

use crate::memory::address::Address;

// ----- Structs ----- //

pub struct Stack {
    values: Vec<Address>,
}

impl Stack {
    pub fn new() -> Self {
        return Stack { values: Vec::new() };
    }

    pub fn push(&mut self, address: Address) {
        self.values.push(address);
    }

    pub fn pop(&mut self) -> Address {
        return self.values.pop().expect("Call stack is empty.");
    }
}
