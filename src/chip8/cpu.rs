use std::mem;

use super::display::{Display, HEIGHT, WIDTH};
use super::memory::Memory;

use rand::Rng;

const OPCODE_SIZE: u16 = 2;
const F: usize = 15;

pub struct Cpu {
    // 16 registers, often indexed as Vx
    regs: [u8; 16],
    // index register
    i: u16,
    // program counter
    pc: u16,
    // stack pointer
    sp: u8,
    // stack
    stack: [u16; 16],
    // delay timer
    dt: u8,
    // stack timer
    st: u8,
    // keys
    keys: [bool; 16],
    // key flags
    waiting_for_press: bool,
    key_reg: u8,
}

enum ProgramCounter {
    Next,
    Skip,
    JumpTo(u16),
}

struct Opcode {
    // nibbles of opcode
    // hi, x, y, n
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
            keys: [false; 16],
            waiting_for_press: false,
            key_reg: 0,
        }
    }

    pub fn execute_next_instruction(&mut self, mem: &mut Memory, display: &mut Display) {
        // build up opcode primitives
        let hi = mem.read_byte(self.pc) as u16;
        let lo = mem.read_byte(self.pc + 1) as u16;

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
            } => self.op_ld_bcd(opcode.x, mem),
            Opcode {
                h: 0xF,
                x: _,
                y: 5,
                n: 5,
            } => self.op_str_regs(opcode.x, mem),
            Opcode {
                h: 0xF,
                x: _,
                y: 6,
                n: 5,
            } => self.op_ld_all_regs(opcode.x, mem),
            _ => ProgramCounter::Next,
        };

        println!("executing {:#X}", instr);
        match update {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += OPCODE_SIZE + OPCODE_SIZE,
            ProgramCounter::JumpTo(addr) => self.pc = addr,
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

    // The interpreter sets the program counter to the address at the top of the
    // stack, then subtracts 1 from the stack pointer.
    fn op_ret(&mut self) -> ProgramCounter {
        self.sp -= 1;
        ProgramCounter::JumpTo(self.stack[self.sp as usize])
    }

    // 1nnn - JP addr
    // Jump to location nnn.

    // The interpreter sets the program counter to nnn.
    fn op_jp_addr(&self, nnn: u16) -> ProgramCounter {
        ProgramCounter::JumpTo(nnn)
    }

    // 2nnn - CALL addr
    // Call subroutine at nnn.

    // The interpreter increments the stack pointer, then puts the current PC on
    // the top of the stack. The PC is then set to nnn.
    fn op_call(&mut self, nnn: u16) -> ProgramCounter {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        ProgramCounter::JumpTo(nnn)
    }

    // 3xkk - SE Vx, byte
    // Skip next instruction if Vx = kk.

    // The interpreter compares register Vx to kk, and if they are equal,
    // increments the program counter by 2.
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

    // The interpreter compares register Vx to kk, and if they are not equal,
    // increments the program counter by 2.
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

    // The interpreter compares register Vx to register Vy, and if they are
    // equal, increments the program counter by 2.
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
    fn op_ld_byte(&mut self, x: u8, kk: u8) -> ProgramCounter {
        self.regs[x as usize] = kk;
        ProgramCounter::Next
    }

    // 7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.

    // Adds the value kk to the value of register Vx, then stores the result in
    // Vx.
    fn op_add_byte(&mut self, x: u8, kk: u8) -> ProgramCounter {
        self.regs[x as usize] += kk;
        ProgramCounter::Next
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.

    // Stores the value of register Vy in register Vx.
    fn op_ld_regs(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.regs[x as usize] = self.regs[y as usize];
        ProgramCounter::Next
    }

    // 8xy1 - OR Vx, Vy
    // Set Vx = Vx OR Vy.

    // Performs a bitwise OR on the values of Vx and Vy, then stores the result
    // in Vx. A bitwise OR compares the corrseponding bits from two values, and
    // if either bit is 1, then the same bit in the result is also 1. Otherwise,
    // it is 0.
    fn op_or(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.regs[x as usize] |= self.regs[y as usize];
        ProgramCounter::Next
    }

    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx AND Vy.

    // Performs a bitwise AND on the values of Vx and Vy, then stores the result
    // in Vx. A bitwise AND compares the corrseponding bits from two values, and
    // if both bits are 1, then the same bit in the result is also 1. Otherwise,
    // it is 0.
    fn op_and(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.regs[x as usize] &= self.regs[y as usize];
        ProgramCounter::Next
    }

    // 8xy3 - XOR Vx, Vy
    // Set Vx = Vx XOR Vy.

    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
    // the result in Vx. An exclusive OR compares the corrseponding bits from
    // two values, and if the bits are not both the same, then the corresponding
    // bit in the result is set to 1. Otherwise, it is 0.
    fn op_xor(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.regs[x as usize] ^= self.regs[y as usize];
        ProgramCounter::Next
    }

    // 8xy4 - ADD Vx, Vy
    // Set Vx = Vx + Vy, set VF = carry.

    // The values of Vx and Vy are added together. If the result is greater than
    // 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits
    // of the result are kept, and stored in Vx.
    fn op_add_regs(&mut self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize] as u16;
        let vy = self.regs[y as usize] as u16;

        let sum = vx + vy;
        self.regs[x as usize] = sum as u8;
        self.regs[F] = if sum > 255 { 1 } else { 0 };
        ProgramCounter::Next
    }

    // 8xy5 - SUB Vx, Vy
    // Set Vx = Vx - Vy, set VF = NOT borrow.

    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from
    // Vx, and the results stored in Vx.
    fn op_sub(&mut self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        let vy = self.regs[y as usize];

        self.regs[F] = if vx > vy { 1 } else { 0 };
        self.regs[x as usize] = vx.wrapping_sub(vy);
        ProgramCounter::Next
    }

    //8xy6 - SHR Vx {, Vy}
    // Set Vx = Vx SHR 1.

    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise
    // 0. Then Vx is divided by 2.
    fn op_shr(&mut self, x: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        self.regs[F] = if vx & 1 == 1 { 1 } else { 0 };
        self.regs[x as usize] >>= 1;
        ProgramCounter::Next
    }

    // 8xy7 - SUBN Vx, Vy
    // Set Vx = Vy - Vx, set VF = NOT borrow.

    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from
    // Vy, and the results stored in Vx.
    fn op_subn(&mut self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        let vy = self.regs[y as usize];

        self.regs[F] = if vy > vx { 1 } else { 0 };
        self.regs[x as usize] = vy.wrapping_sub(vx);
        ProgramCounter::Next
    }

    // 8xyE - SHL Vx {, Vy}
    // Set Vx = Vx SHL 1.

    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to
    // 0. Then Vx is multiplied by 2.
    fn op_shl(&mut self, x: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        self.regs[F] = if vx >> 7 == 1 { 1 } else { 0 };
        self.regs[x as usize] <<= 1;
        ProgramCounter::Next
    }

    // 9xy0 - SNE Vx, Vy
    // Skip next instruction if Vx != Vy.

    // The values of Vx and Vy are compared, and if they are not equal, the
    // program counter is increased by 2.
    fn op_sne_regs(&self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        let vy = self.regs[y as usize];
        if vx != vy {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    // Annn - LD I, addr
    // Set I = nnn.

    // The value of register I is set to nnn.
    fn op_ld_i(&mut self, nnn: u16) -> ProgramCounter {
        self.i = nnn;
        ProgramCounter::Next
    }

    // Cxkk - RND Vx, byte
    // Set Vx = random byte AND kk.

    // The interpreter generates a random number from 0 to 255, which is then
    // ANDed with the value kk. The results are stored in Vx. See instruction
    // 8xy2 for more information on AND.
    fn op_rand(&mut self, x: u8, kk: u8) -> ProgramCounter {
        let n: u8 = rand::thread_rng().gen();
        self.regs[x as usize] = n & kk;
        ProgramCounter::Next
    }

    // TODO: Dxyn - DRW Vx, Vy, nibble
    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

    // The interpreter reads n bytes from memory, starting at the address stored
    // in I. These bytes are then displayed as sprites on screen at coordinates
    // (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any
    // pixels to be erased, VF is set to 1, otherwise it is set to 0. If the
    // sprite is positioned so part of it is outside the coordinates of the
    // display, it wraps around to the opposite side of the screen. See
    // instruction 8xy3 for more information on XOR, and section 2.4, Display,
    // for more information on the Chip-8 screen and sprites.
    fn op_display_sprite(&self, x: u8, y: u8, n: u8, display: &mut Display) -> ProgramCounter {
        self.regs[F] = 0;
        let vx = self.regs[x as usize];
        let vy = self.regs[y as usize];
        for byte in 0..n {
            // TODO
        }
        ProgramCounter::Next
    }

    // Ex9E - SKP Vx
    // Skip next instruction if key with the value of Vx is pressed.

    // Checks the keyboard, and if the key corresponding to the value of Vx is
    // currently in the down position, PC is increased by 2.
    fn op_skp(&self, x: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        if self.keys[vx as usize] {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    // ExA1 - SKNP Vx
    // Skip next instruction if key with the value of Vx is not pressed.

    // Checks the keyboard, and if the key corresponding to the value of Vx is
    // currently in the up position, PC is increased by 2.
    fn op_sknp(&self, x: u8) -> ProgramCounter {
        let vx = self.regs[x as usize];
        if !self.keys[vx as usize] {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    // Fx07 - LD Vx, DT
    // Set Vx = delay timer value.

    // The value of DT is placed into Vx.
    fn op_ld_dt(&mut self, x: u8) -> ProgramCounter {
        self.regs[x as usize] = self.dt;
        ProgramCounter::Next
    }

    // Fx0A - LD Vx, K
    // Wait for a key press, store the value of the key in Vx.

    // All execution stops until a key is pressed, then the value of that key is
    // stored in Vx.
    fn op_ld_store_key(&mut self, x: u8) -> ProgramCounter {
        self.waiting_for_press = true;
        self.key_reg = x;
        ProgramCounter::Next
    }

    // Fx15 - LD DT, Vx
    // Set delay timer = Vx.

    // DT is set equal to the value of Vx.
    fn op_ld_vx(&mut self, x: u8) -> ProgramCounter {
        self.dt = self.regs[x as usize];
        ProgramCounter::Next
    }

    // Fx18 - LD ST, Vx
    // Set sound timer = Vx.

    // ST is set equal to the value of Vx.
    fn op_ld_st(&mut self, x: u8) -> ProgramCounter {
        self.st = self.regs[x as usize];
        ProgramCounter::Next
    }

    // Fx1E - ADD I, Vx
    // Set I = I + Vx.

    // The values of I and Vx are added, and the results are stored in I.
    fn op_add_i(&mut self, x: u8) -> ProgramCounter {
        self.i += self.regs[x as usize] as u16;
        ProgramCounter::Next
    }

    // Fx29 - LD F, Vx
    // Set I = location of sprite for digit Vx.

    // The value of I is set to the location for the hexadecimal sprite
    // corresponding to the value of Vx. See section 2.4, Display, for more
    // information on the Chip-8 hexadecimal font.
    fn op_ld_digit(&mut self, x: u8) -> ProgramCounter {
        self.i = (self.regs[x as usize] as u16) * 5;
        ProgramCounter::Next
    }

    // Fx33 - LD B, Vx
    // Store BCD representation of Vx in memory locations I, I+1, and I+2.

    // The interpreter takes the decimal value of Vx, and places the hundreds
    // digit in memory at location in I, the tens digit at location I+1, and the
    // ones digit at location I+2.
    fn op_ld_bcd(&self, x: u8, mem: &Memory) -> ProgramCounter {
        let vx = self.regs[x as usize];
        mem.write_byte(self.i, vx / 100);
        mem.write_byte(self.i + 1, (vx % 100) / 10);
        mem.write_byte(self.i + 2, vx % 10);
        ProgramCounter::Next
    }

    // Fx55 - LD [I], Vx
    // Store registers V0 through Vx in memory starting at location I.

    // The interpreter copies the values of registers V0 through Vx into memory,
    // starting at the address in I.
    fn op_str_regs(&mut self, x: u8, mem: &mut Memory) -> ProgramCounter {
        for j in 0..(x as usize) {
            mem.write_byte(self.i + (j as u16), self.regs[j]);
        }
        ProgramCounter::Next
    }

    // Fx65 - LD Vx, [I]
    // Read registers V0 through Vx from memory starting at location I.

    // The interpreter reads values from memory starting at location I into
    // registers V0 through Vx.
    fn op_ld_all_regs(&mut self, x: u8, mem: &Memory) -> ProgramCounter {
        for j in 0..(x as usize) {
            self.regs[j] = mem.read_byte(self.i + (j as u16));
        }
        ProgramCounter::Next
    }
}
