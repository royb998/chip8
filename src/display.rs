// ----- Imports ----- //

use std::io::stdout;
use crossterm::{cursor, execute, style, terminal};

// ----- Consts ----- //

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

const SPRITE_WIDTH: usize = 8;
const SPRITE_HEIGHT: usize = 15;

// ----- Structs ----- //


pub struct Display {
    grid: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

pub struct Sprite {
    pixels: [[bool; SPRITE_WIDTH]; SPRITE_HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        return Display {
            grid: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        };
    }

    pub fn clear(&mut self) {
        let _ = execute!(stdout(), terminal::Clear(terminal::ClearType::All));
        drop(self.grid);
        self.grid = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    /// Add the given sprite to the display.
    /// Pixels from the given `sprite` are xorred with existing pixels, so that
    /// 1s switch a pixel, and 0s have no effect.
    ///
    /// Returns `true` if any operation resulted in a pixel getting turned off,
    /// `false` otherwise.
    pub fn add_sprite(&mut self, sprite: &Sprite, mut x: usize, mut y: usize) -> bool {
        x = x % DISPLAY_WIDTH;
        y = y % DISPLAY_HEIGHT;
        let mut result = false;

        for i in 0..SPRITE_WIDTH {
            if x + i >= DISPLAY_WIDTH { break; }

            for j in 0..SPRITE_HEIGHT {
                if y + j >= DISPLAY_HEIGHT { break; }

                let current = self.grid[y + j][x + i];
                let pixel = sprite.get_pixel(i, j);
                result |= current & pixel;
                self.grid[y + j][x + i] = current ^ pixel;

                if pixel {
                    let mut s = String::from("█");
                    if !current ^ pixel {
                        s = String::from(" ");
                    }
                    let moveto = cursor::MoveTo((x + i) as u16, (y + j) as u16);
                    let _ = execute!(stdout(), moveto, style::Print(s));
                }
            }
        }
        return result;
    }
}

impl Sprite {
    pub fn from(bytes: Vec<u8>) -> Sprite {
        assert!(bytes.len() <= SPRITE_HEIGHT);

        let mut pixels: [[bool; SPRITE_WIDTH]; SPRITE_HEIGHT] = [
            [false; SPRITE_WIDTH]; SPRITE_HEIGHT
        ];

        for (i, byte) in bytes.iter().enumerate() {
            if i >= SPRITE_HEIGHT { break; }

            for j in 0..SPRITE_WIDTH {
                pixels[i][SPRITE_WIDTH - j - 1] = (byte & (1 << j)) > 0;
            }
        }

        return Sprite {
            pixels,
        };
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        return self.pixels[y][x];
    }
}
