// ----- Modules ----- //

mod cpu;
mod display;
mod keyboard;
mod memory;
mod registers;
mod stack;
mod timers;

// ----- Imports ----- //

use std::env;
use std::io::stdout;
use crossterm::{execute, terminal};

// ----- Main Entry Point ----- //

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing target executable!");
        return;
    }

    let _ = execute!(stdout(), terminal::Clear(terminal::ClearType::All));
    let mut chip8 = cpu::CPU::new(&args[1]);
    chip8.run();
}
