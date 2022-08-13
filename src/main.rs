use chip8::Chip8;
use std::{env, fs::File, io::Read, thread, time::Duration};

mod chip8;

fn main() {
    // 1. Read ROM from arguments
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args.get(1).unwrap()).unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");

    // 2. Setup Chip8
    let mut chip8: Chip8 = Chip8::new();
    chip8.load_rom(&data);
    loop {
        chip8.execute_next_instruction();
        thread::sleep(Duration::from_millis(2));
    }
}
