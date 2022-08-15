# A *Rusty* CHIP-8 Emulator

### 🌊🌊🌊🌊 🏝️ 🦀 🦀 🦀 🏝️ 🌊🌊🌊🌊

[](tetris.png)

I first wanted to build a GBA emulator, since I've grown up on those, but it seems like a lot for a first-time emulator (and honestly, implementing [every single instruction](https://meganesulli.com/generate-gb-opcodes/) would take forever). I'm also completely new to Rust, so building a CHIP-8 emulator seems like a good place to start. Overall, this is a fantastic systems project to learn a new language (and have something to show at the end of it all).

I primarily tested this emulator on TETRIS and PONG, so try those for the best experience.

Note that it seems like the key debouncer within `minifb` is not the best, so sometimes keys may not be received properly. This is a first time Rust and building an emulator project I've done, so expect some quirks.

### Resources Used:
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#1.0)
- [Colin Eberhardt's WASM Rust Chip-8 Output](https://colineberhardt.github.io/wasm-rust-chip8/web/) 

    *this one helped me make sure my emulator was reading and parsing bits correctly*
- [Why learn Rust at all?](https://www.youtube.com/watch?v=4YU_r70yGjQ)