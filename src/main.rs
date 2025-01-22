use std::env;

use crate::cpu::CPU;

mod cpu;
mod memory;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom = &args[1];
    println!("{rom}");

    let mut cpu = CPU::new();
    cpu.tick();
    cpu.tick();
    cpu.tick();
}
