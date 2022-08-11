use super::display::{Display, HEIGHT, WIDTH};
use super::memory::Memory;

use rand::Rng;

const OPCODE_SIZE: u16 = 2;
const F: usize = 15;

pub struct Cpu {
    regs: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    dt: u8,
    st: u8,
}

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

struct Opcode {
    h: u8,
    x: u8,
    y: u8,
    n: u8,
}

impl Opcode {
    pub fn parse(instr: u16) -> Opcode {
        Opcode {
            h: ((instr & 0xF000) >> 12) as u8,
            x: ((instr & 0x0F00) >> 8) as u8,
            y: ((instr & 0x00F0) >> 4) as u8,
            n: (instr & 0x000F) as u8,
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            dt: 0,
            st: 0,
        }
    }

    pub fn execute_next_instruction(&mut self, mem: &Memory, display: &mut Display) {
        // build up opcode primitives
        let lo = mem.read_byte(self.pc) as u16;
        let hi = mem.read_byte(self.pc + 1) as u16;

        let instr = (hi << 8) | lo;

        let opcode = Opcode::parse(instr);

        let nnn = (instr & 0x0FFF) as u16;
        let kk = (instr & 0x00FF) as u8;

        // process opcode
        let update = match opcode {
            Opcode {
                h: 0,
                x: 0,
                y: 0xE,
                n: 0,
            } => self.op_cls(display),
            Opcode {
                h: 0,
                x: 0,
                y: 0xE,
                n: 0xE,
            } => self.op_ret(),
            Opcode {
                h: 1,
                x: _,
                y: _,
                n: _,
            } => self.op_jp_addr(nnn),
            Opcode {
                h: 2,
                x: _,
                y: _,
                n: _,
            } => self.op_call(nnn),
            Opcode {
                h: 3,
                x: _,
                y: _,
                n: _,
            } => self.op_se_byte(opcode.x, kk),
            Opcode {
                h: 4,
                x: _,
                y: _,
                n: _,
            } => self.op_sne_byte(opcode.x, kk),
            Opcode {
                h: 5,
                x: _,
                y: _,
                n: 0,
            } => self.op_se_reg(opcode.x, opcode.y),
            Opcode {
                h: 6,
                x: _,
                y: _,
                n: _,
            } => self.op_ld_byte(opcode.x, kk),
            Opcode {
                h: 7,
                x: _,
                y: _,
                n: _,
            } => self.op_add_byte(opcode.x, kk),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 0,
            } => self.op_ld_regs(opcode.x, opcode.y),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 1,
            } => self.op_or(opcode.x, opcode.y),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 2,
            } => self.op_and(opcode.x, opcode.y),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 3,
            } => self.op_xor(opcode.x, opcode.y),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 4,
            } => self.op_add_regs(opcode.x, opcode.y),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 5,
            } => self.op_sub(opcode.x, opcode.y),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 6,
            } => self.op_shr(opcode.x),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 7,
            } => self.op_subn(opcode.x, opcode.y),
            Opcode {
                h: 8,
                x: _,
                y: _,
                n: 0xE,
            } => self.op_shl(opcode.x),
            Opcode {
                h: 9,
                x: _,
                y: _,
                n: 0,
            } => self.op_sne_regs(opcode.x, opcode.y),
            Opcode {
                h: 0xA,
                x: _,
                y: _,
                n: _,
            } => self.op_ld_i(nnn),
            Opcode {
                h: 0xB,
                x: _,
                y: _,
                n: _,
            } => self.op_jp_addr(nnn + self.regs[0] as u16),
            Opcode {
                h: 0xC,
                x: _,
                y: _,
                n: _,
            } => self.op_rand(opcode.x, kk),
            Opcode {
                h: 0xD,
                x: _,
                y: _,
                n: _,
            } => self.op_display_sprite(opcode.x, opcode.y, display),
            Opcode {
                h: 0xE,
                x: _,
                y: 9,
                n: E,
            } => self.op_skp(opcode.x),
            Opcode {
                h: 0xE,
                x: _,
                y: 0xA,
                n: 1,
            } => self.op_sknp(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 0,
                n: 7,
            } => self.op_ld_dt(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 0,
                n: 0xA,
            } => self.op_ld_store_key(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 1,
                n: 5,
            } => self.op_ld_vx(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 1,
                n: 8,
            } => self.op_ld_st(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 1,
                n: 0xE,
            } => self.op_add_i(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 2,
                n: 9,
            } => self.op_ld_digit(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 3,
                n: 3,
            } => self.op_ld_bcd(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 5,
                n: 5,
            } => self.op_str_regs(opcode.x),
            Opcode {
                h: 0xF,
                x: _,
                y: 6,
                n: 5,
            } => self.op_ld_regs(opcode.x),
            _ => ProgramCounter::Skip,
        };

        println!("executing {:#X}", instr);
        match update {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += OPCODE_SIZE + OPCODE_SIZE,
            ProgramCounter::Jump(addr) => self.pc = addr,
        }
    }

    // 00E0 - CLS
    // Clear the display.
    fn op_cls(&self, display: &mut Display) -> ProgramCounter {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                display.write(x, y, 0);
            }
        }
        display.will_need_update();
        ProgramCounter::Next
    }

    // ---- OPCODE INTERPRETATION ----

    // 00EE - RET
    // Return from a subroutine.
    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn op_ret(&mut self) -> ProgramCounter {
        self.sp -= 1;
        ProgramCounter::Jump(self.stack[self.sp as usize])
    }

    // 1nnn - JP addr
    // Jump to location nnn.
    // The interpreter sets the program counter to nnn.
    fn op_jp_addr(&self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }

    // 2nnn - CALL addr
    // Call subroutine at nnn.
    // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn op_call(&self, nnn: u16) -> ProgramCounter {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        ProgramCounter::Jump(nnn)
    }

    // 3xkk - SE Vx, byte
    // Skip next instruction if Vx = kk.
    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn op_se_byte(&self, x: u8, kk: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        if vx == kk {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    // 4xkk - SNE Vx, byte
    // Skip next instruction if Vx != kk.
    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn op_sne_byte(&self, x: u8, kk: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        if vx != kk {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    // 5xy0 - SE Vx, Vy
    // Skip next instruction if Vx = Vy.
    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn op_se_reg(&self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        let vy = self.regs[y as usize];
        if vx == vy {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    // 6xkk - LD Vx, byte
    // Set Vx = kk.
    // The interpreter puts the value kk into register Vx.
    fn op_ld_byte(&self, x: u8, kk: u8) -> ProgramCounter {
        self.regs[x as usize] = kk;
        ProgramCounter::Next
    }

    // 7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn op_add_byte(&self, x: u8, kk: u8) -> ProgramCounter {
        self.regs[x as usize] += kk;
        ProgramCounter::Next
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.
    // Stores the value of register Vy in register Vx.
    fn op_ld_regs(&self, x: u8, y: u8) -> ProgramCounter {
        self.regs[x as usize] = self.regs[y as usize];
        ProgramCounter::Next
    }

    fn op_or(&self, x: u8, y: u8) -> ProgramCounter {
        self.regs[x as usize] |= self.regs[y as usize];
        ProgramCounter::Next
    }

    fn op_and(&self, x: u8, y: u8) -> ProgramCounter {
        self.regs[x as usize] &= self.regs[y as usize];
        ProgramCounter::Next
    }

    fn op_xor(&self, x: u8, y: u8) -> ProgramCounter {
        self.regs[x as usize] ^= self.regs[y as usize];
        ProgramCounter::Next
    }

    fn op_add_regs(&self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize] as u16;
        let vy = self.regs[y as usize] as u16;

        let sum = vx + vy;
        self.regs[x as usize] = sum as u8;
        self.regs[F] = if sum > 255 { 1 } else { 0 };
        ProgramCounter::Next
    }

    fn op_sub(&self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        let vy = self.regs[y as usize];

        self.regs[F] = if vx > vy { 1 } else { 0 };
        self.regs[x as usize] = vx.wrapping_sub(vy);
        ProgramCounter::Next
    }

    fn op_shr(&self, x: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        self.regs[F] = if vx & 1 == 1 { 1 } else { 0 };
        self.regs[x as usize] >>= 1;
        ProgramCounter::Next
    }

    fn op_subn(&self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        let vy = self.regs[y as usize];

        self.regs[F] = if vy > vx { 1 } else { 0 };
        self.regs[x as usize] = vy.wrapping_sub(vx);
        ProgramCounter::Next
    }

    fn op_shl(&self, x: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        self.regs[F] = if vx >> 7 == 1 { 1 } else { 0 };
        self.regs[x as usize] <<= 1;
        ProgramCounter::Next
    }

    fn op_sne_regs(&self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        let vy = self.regs[y as usize];
        if vx != vy {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    fn op_ld_i(&self, nnn: u16) -> ProgramCounter {
        self.i = nnn;
        ProgramCounter::Next
    }

    fn op_rand(&self, x: u8, kk: u8) -> ProgramCounter {
        let n: u8 = rand::thread_rng().gen();
        self.regs[x as usize] = n & kk;
        ProgramCounter::Next
    }

    fn op_ld_dt(&self, x: u8) -> ProgramCounter {
        self.regs[x as usize] = self.dt;
        ProgramCounter::Next
    }

    fn op_ld_vx(&self, x: u8) -> ProgramCounter {
        self.dt = self.regs[x as usize];
        ProgramCounter::Next
    }

    fn op_ld_st(&self, x: u8) -> ProgramCounter {
        self.st = self.regs[x as usize];
        ProgramCounter::Next
    }

    fn op_add_i(&self, x: u8) -> ProgramCounter {
        self.i += self.regs[x as usize] as u16;
        ProgramCounter::Next
    }

    fn op_ld_digit(&self, x: u8) -> ProgramCounter {
        self.i = (self.regs[x as usize] as u16) * 5;
        ProgramCounter::Next
    }
}
