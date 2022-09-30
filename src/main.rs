// ----- Modules ----- //

mod cpu;
mod display;
mod memory;
mod registers;
mod stack;

// ----- Imports ----- //

use std::env;

// ----- Main Entry Point ----- //

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing target exe!");
    }

    let mut cpu = CPU::new(&args[1]);
    cpu.run();
}
