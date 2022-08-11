use super::memory::Memory;

pub struct Cpu {
    regs: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn execute_next_instruction(&self, mem: &Memory) {
        // get opcode
        let lo = mem.read_byte(self.pc) as u16;
        let hi = mem.read_byte(self.pc + 1) as u16;

        let opcode = (hi << 8) | lo;

        // process opcode
        println!("executing {:#X}", opcode);
    }
}
