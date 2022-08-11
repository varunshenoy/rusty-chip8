use chip8::Chip8;

mod chip8;

fn main() {
    // 1. Read ROM from arguments

    // 2. Setup Chip8
    let chip8: Chip8 = Chip8::new();
    chip8.execute_next_instruction();

    println!("Hello, world!");
}
