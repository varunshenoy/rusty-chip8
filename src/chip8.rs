mod cpu;
mod memory;

const PROGRAM_OFFSET: u16 = 0x200;

pub struct Chip8 {
    memory: memory::Memory,
    cpu: cpu::Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: memory::Memory::new(),
            cpu: cpu::Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        for idx in 0..rom.len() {
            self.memory
                .write_byte(PROGRAM_OFFSET + (idx as u16), rom[idx]);
        }
    }

    pub fn execute_next_instruction(&self) {
        self.cpu.execute_next_instruction(&self.memory);
    }
}