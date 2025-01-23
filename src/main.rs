use std::{env, fmt::Display};

use crate::{cpu::CPU, display::C8Display};

mod cpu;
mod display;
mod memory;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom = &args[1];
    println!("{rom}");

    let mut cpu = CPU::new();
    let mut display = C8Display::new();
    cpu.tick();
}
