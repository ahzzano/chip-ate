use std::usize;

#[derive(Debug)]
pub struct Memory {
    memory_map: [u8; 0xFFF],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory_map: [0; 0xFFF],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory_map[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.memory_map[addr as usize] = value;
    }
}
