// ----- Structs ----- //

// TODO: Consider handling the stack in the memory.
pub struct Stack {
    values: Vec<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        return Stack { values: Vec::new() };
    }

    pub fn push(&mut self, address: u16) {
        self.values.push(address);
    }

    // TODO: Raise errors and remove Option.
    pub fn pop(&mut self) -> Option<u16> {
        return self.values.pop();
    }
}
