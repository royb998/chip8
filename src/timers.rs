// ----- Imports ----- //

use std::{thread, time};
use std::sync::{Arc, Mutex};
use spin_sleep::sleep;

// ----- Consts ----- //

const DURATION: time::Duration = time::Duration::from_millis(16);

// ----- Structs ----- //

pub struct Timer {
    value: Arc<Mutex<u8>>,
}

impl Timer {
    pub fn new() -> Self {
        let value = Arc::new(Mutex::new(0));
        let other = value.clone();
        thread::spawn(|| count(other));
        return Timer { value };
    }

    pub fn get(&self) -> u8 {
        return *self.value.lock().unwrap();
    }

    pub fn set(&mut self, value: u8) {
        let _ = value;
        let mut current = self.value.lock().unwrap();
        *current = value;
    }
}

fn count(m: Arc<Mutex<u8>>) {
    loop {
        sleep(DURATION);

        {
            let mut value = m.lock().unwrap();
            if *value > 0 {
                *value -= 1;
            }
        }
    }
}
