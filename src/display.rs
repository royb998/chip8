// ----- Imports ----- //

use std::process::Command;

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

    pub fn show(&self) {
        Command::new("clear").status();  // TODO: Handle possible error?
        for row in self.grid.iter() {
            for pixel in row.iter() {
                if *pixel {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn clear(&mut self) {
        for row in 0..DISPLAY_HEIGHT {
            for col in 0..DISPLAY_WIDTH {
                self.grid[row][col] = false;
            }
        }
    }

    pub fn add_sprite(&mut self, sprite: &Sprite, x: usize, y: usize) {
        for i in 0..SPRITE_WIDTH {
            if x + i >= DISPLAY_WIDTH { break; }

            for j in 0..SPRITE_HEIGHT {
                if y + j >= DISPLAY_HEIGHT { break; }

                let current = self.grid[y + j][x + i];
                self.grid[y + j][x + i] = current ^ sprite.get_pixel(i, j);
            }
        }
    }
}

impl Sprite {
    pub fn from(bytes: Vec<u8>) -> Sprite {
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
