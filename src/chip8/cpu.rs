use super::display::Display;
use super::memory::Memory;

pub struct Cpu {
    regs: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
}

struct Opcode {
    h: u8,
    x: u8,
    y: u8,
    n: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn execute_next_instruction(&mut self, mem: &Memory, display: &Display) {
        // build up opcode primitives
        let lo = mem.read_byte(self.pc) as u16;
        let hi = mem.read_byte(self.pc + 1) as u16;

        let instr = (hi << 8) | lo;

        let opcode = Opcode {
            h: ((instr & 0xF000) >> 12) as u8,
            x: ((instr & 0x0F00) >> 8) as u8,
            y: ((instr & 0x00F0) >> 4) as u8,
            n: (instr & 0x000F) as u8,
        };

        let nnn = (instr & 0x0FFF) as usize;
        let kk = (instr & 0x00FF) as u8;

        // process opcode
        match opcode {
            Opcode {
                h: 11,
                x: 4,
                y: 10,
                n: 2,
            } => println!("hello"),
            _ => println!("goodbye!"),
        }

        println!("executing {:#X}", instr);
        self.pc += 1;
    }
}
