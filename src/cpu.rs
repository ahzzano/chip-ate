use crate::memory::Memory;

#[derive(Debug)]
pub struct CPU {
    registers: [u8; 16],
    pc: u16,
    memory: Box<Memory>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            // Chip-8 Programs are loaded at the 0x0600 Address
            pc: 0x600,
            memory: Box::new(Memory::new()),
        }
    }

    pub fn tick(&mut self) {
        self.pc += 2;
    }
}
