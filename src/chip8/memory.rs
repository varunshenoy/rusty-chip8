const MEM_SIZE: usize = 0xFFF;
const RESERVED: u16 = 0x200;

#[derive(Debug)]
pub struct Memory {
    data: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            data: [0; MEM_SIZE],
        };

        let number_sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut curr_idx = 0;
        for num in number_sprites {
            for byte in num {
                memory.data[curr_idx] = byte;
                curr_idx += 1;
            }
        }

        // println!("MEMORY: {:?}", memory.data);

        memory
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        if address < RESERVED {
            panic!(
                "ERROR: Tried to write data in RESERVED region at address {}",
                address
            )
        }
        self.data[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
}
