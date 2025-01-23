/*
 *  Main Reference: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0
 */
use std::{mem::offset_of, usize};

use rand::Rng;

use crate::memory::Memory;

enum PCBypass {
    None,
    JumpAddr(u16),
    Offset(u16),
}

#[derive(Debug)]
pub struct CPU {
    registers: [u8; 16],
    pc: u16,
    vf: u8,
    ind: u16,
    memory: Box<Memory>,
    sp: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            // Chip-8 Programs are loaded at the 0x0600 Address
            pc: 0x600,
            vf: 0x00,
            ind: 0x0000,
            memory: Box::new(Memory::new()),
            sp: 0x0052,
        }
    }

    // Mostly for teseting purposes
    fn load_register_file(&mut self, register_file: [u8; 16]) {
        self.registers = register_file;
    }

    pub fn tick(&mut self) {
        let hi = self.memory.read(self.pc) as u16;
        let lo = self.memory.read(self.pc + 1) as u16;

        let instruction: u16 = hi << 8 | lo;
        let post_pc = self.execute_instruction(instruction);

        match post_pc {
            PCBypass::Offset(offset) => {
                self.pc += offset;
            }
            PCBypass::None => {
                self.pc += 2;
            }
            PCBypass::JumpAddr(addr) => {
                self.pc = addr;
            }
        }
    }

    fn execute_instruction(&mut self, instruction: u16) -> PCBypass {
        let instr_type = (instruction & 0xF000) >> 12;
        let mut def_ret = PCBypass::None;
        match instr_type {
            // JMP instruction
            1 => {
                let jump_addr = instruction & 0x0FFF;
                def_ret = PCBypass::JumpAddr(jump_addr);
            }
            2 => {
                let subr = instruction & 0x0FFF;
                self.sp += 1;
            }
            // SET instruction
            6 => {
                let value_to_inject = (instruction & 0xFF) as u8;
                let reg_x = ((instruction & 0x0F00) >> 8) as u8;
                self.registers[reg_x as usize] = value_to_inject;
            }
            // ADD instruction
            7 => {
                let value_to_inject = (instruction & 0xFF) as u8;
                let reg_x = ((instruction & 0x0F00) >> 8) as u8;
                self.registers[reg_x as usize] += value_to_inject;
            }
            8 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let reg_y = ((instruction & 0x00F0) >> 4) as usize;
                let op = (instruction & 0x000F) as u8;

                let x = self.registers[reg_x as usize];
                let y = self.registers[reg_y as usize];

                match op {
                    1 => {
                        // OR
                        self.registers[reg_x] = x | y;
                    }
                    2 => {
                        // AND
                        self.registers[reg_x] = x & y;
                    }
                    3 => {
                        // XOR
                        self.registers[reg_x] = x ^ y;
                    }
                    4 => {
                        // ADD
                        self.registers[reg_x] = x + y;
                        self.vf = if x as u16 + y as u16 > 255 { 1 } else { 0 };
                    }
                    5 => {
                        // SUB
                        self.registers[reg_x] = x - y;
                        self.vf = if x > y { 1 } else { 0 };
                    }
                    6 => {
                        let cur_x = self.registers[reg_x];
                        if (cur_x & 1) != 0 {
                            self.vf = 1;
                        }
                        self.registers[reg_x] >>= 1;
                    }
                    7 => {
                        // SUB
                        self.registers[reg_x] = y - x;
                        self.vf = if y > x { 1 } else { 0 };
                    }
                    0xE => {
                        let cur_x = self.registers[reg_x];
                        if (cur_x & 1) != 0 {
                            self.vf = 1;
                        }
                        self.registers[reg_x] <<= 1;
                    }
                    _ => {}
                }
            }
            0xA => {
                let addr = instruction & 0x0FFF;
                self.ind = addr;
            }
            0xC => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let nn = (instruction & 0xFF) as u8;
                self.registers[reg_x] = rand::thread_rng().gen::<u8>() & nn;
            }
            0xD => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let reg_y = ((instruction & 0x00F0) >> 4) as usize;

                let x = self.registers[reg_x];
                let y = self.registers[reg_y];
                let n = (instruction & 0xF) as usize;

                todo!("Requires the Display to work already")
            }
            _ => {}
        }

        def_ret
    }
}

#[cfg(test)]
mod test {
    use super::CPU;

    #[test]
    fn add_to_register() {
        let mut cpu = CPU::new();

        cpu.registers[1] = 1;

        cpu.memory.write(0x600, 0x70);
        cpu.memory.write(0x601, 0x69);

        cpu.memory.write(0x602, 0x71);
        cpu.memory.write(0x603, 0xC0);

        cpu.tick();
        cpu.tick();

        assert_eq!(cpu.registers[0], 0x69);
        assert_eq!(cpu.registers[1], 0xC1);
    }

    #[test]
    fn set_to_register() {
        let mut cpu = CPU::new();

        cpu.registers[1] = 1;

        cpu.memory.write(0x600, 0x60);
        cpu.memory.write(0x601, 0x69);

        cpu.memory.write(0x602, 0x61);
        cpu.memory.write(0x603, 0xC0);

        cpu.tick();
        cpu.tick();

        assert_eq!(cpu.registers[0], 0x69);
        assert_eq!(cpu.registers[1], 0xC0);
    }

    #[test]
    fn jump_instruction() {
        let mut cpu = CPU::new();
        // jump to 0x-C0D
        cpu.memory.write(0x600, 0x1C);
        cpu.memory.write(0x601, 0x0D);

        // find our way to comeback to 0x600
        cpu.memory.write(0xC0D, 0x16);
        cpu.memory.write(0xC0E, 0x00);

        cpu.tick();
        assert_eq!(cpu.pc, 0x0C0D);
        cpu.tick();
        assert_eq!(cpu.pc, 0x0600);
    }

    #[test]
    fn alu_instructions() {
        let mut cpu = CPU::new();
        cpu.load_register_file([1; 16]);

        // ADD and SUB
        cpu.memory.write(0x600, 0x80);
        cpu.memory.write(0x601, 0x14);
        cpu.memory.write(0x602, 0x82);
        cpu.memory.write(0x603, 0x35);

        // other
        cpu.memory.write(0x604, 0x80);
        cpu.memory.write(0x605, 0x11);
        cpu.memory.write(0x606, 0x82);
        cpu.memory.write(0x607, 0x32);

        cpu.tick();
        cpu.tick();

        let reg0 = cpu.registers[0];
        let reg2 = cpu.registers[2];

        assert_eq!(reg0, 2); // ADD
        assert_eq!(reg2, 0); // SUB

        cpu.load_register_file([1; 16]);
        cpu.registers[1] = 2;
        cpu.registers[2] = 2;
        cpu.registers[3] = 4;

        cpu.tick();
        cpu.tick();

        let reg2 = cpu.registers[2];
        let reg0 = cpu.registers[0];
        assert_eq!(reg2, 2 & 4);
        assert_eq!(reg0, 3)
    }
}
