// ----- Imports ----- //

use std::fmt::{Display, Formatter};
use device_query::{DeviceQuery, DeviceState, Keycode};

// ----- Consts ----- //

pub const MAX_KEY: u8 = 0x0F;
pub const INVALID_KEY: u8 = 0xFF;

pub const VALID_KEYS: [Keycode; 0x10] = [
    Keycode::Key1,
    Keycode::Key2,
    Keycode::Key3,
    Keycode::Key4,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::R,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::F,
    Keycode::Z,
    Keycode::X,
    Keycode::C,
    Keycode::V,
];

// ----- Structs ----- //

#[derive(Eq, PartialEq)]
pub struct Key {
    value: u8,
}

impl Key {
    pub fn get(&self) -> u8 {
        return self.value;
    }
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        if value > 0x0F {
            return Key { value: INVALID_KEY };
        }
        Key { value }
    }
}

impl From<Keycode> for Key {
    fn from(value: Keycode) -> Self {
        match value {
            Keycode::Key1 => { Key { value: 0x1 } }
            Keycode::Key2 => { Key { value: 0x2 } }
            Keycode::Key3 => { Key { value: 0x3 } }
            Keycode::Key4 => { Key { value: 0xC } }
            Keycode::Q => { Key { value: 0x4 } }
            Keycode::W => { Key { value: 0x5 } }
            Keycode::E => { Key { value: 0x6 } }
            Keycode::R => { Key { value: 0xD } }
            Keycode::A => { Key { value: 0x7 } }
            Keycode::S => { Key { value: 0x8 } }
            Keycode::D => { Key { value: 0x9 } }
            Keycode::F => { Key { value: 0xE } }
            Keycode::Z => { Key { value: 0xA } }
            Keycode::X => { Key { value: 0x0 } }
            Keycode::C => { Key { value: 0xB } }
            Keycode::V => { Key { value: 0xF } }
            _ => { Key { value: INVALID_KEY } }
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.value > MAX_KEY {
            write!(f, "Key:{}", self.value)
        } else {
            write!(f, "Key:Invalid")
        }
    }
}

pub fn get_key() -> Option<Key> {
    let ds = DeviceState::new();
    let keys = ds.get_keys();
    let filtered = keys.iter().filter(|key| VALID_KEYS.contains(key));

    if let Some(code) = filtered.last() {
        return Some(Key::from(*code));
    }

    None
}
