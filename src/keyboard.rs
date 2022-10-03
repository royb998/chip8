// ----- Imports ----- //

use std::fmt::{Display, Formatter};
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEventKind};

// ----- Consts ----- //

pub const MAX_KEY: u8 = 0x0F;
pub const INVALID_KEY: u8 = 0xFF;

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

impl From<KeyCode> for Key {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Char('1') => { Key { value: 0x1 } }
            KeyCode::Char('2') => { Key { value: 0x2 } }
            KeyCode::Char('3') => { Key { value: 0x3 } }
            KeyCode::Char('4') => { Key { value: 0xC } }
            KeyCode::Char('q') => { Key { value: 0x4 } }
            KeyCode::Char('w') => { Key { value: 0x5 } }
            KeyCode::Char('e') => { Key { value: 0x6 } }
            KeyCode::Char('r') => { Key { value: 0xD } }
            KeyCode::Char('a') => { Key { value: 0x7 } }
            KeyCode::Char('s') => { Key { value: 0x8 } }
            KeyCode::Char('d') => { Key { value: 0x9 } }
            KeyCode::Char('f') => { Key { value: 0xE } }
            KeyCode::Char('z') => { Key { value: 0xA } }
            KeyCode::Char('x') => { Key { value: 0x0 } }
            KeyCode::Char('c') => { Key { value: 0xB } }
            KeyCode::Char('v') => { Key { value: 0xF } }
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
    let no_wait = Duration::from_secs(0);
    if let Ok(status) = crossterm::event::poll(no_wait) {
        if status {
            let event = crossterm::event::read().unwrap();
            if let Event::Key(ke) = event {
                if ke.kind == KeyEventKind::Press  {
                    return Some(Key::from(ke.code));
                }
            }
        }
    }

    return None;
}
