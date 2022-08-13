extern crate minifb;

use minifb::{Key, Window, WindowOptions};

pub const HEIGHT: usize = 32;
pub const WIDTH: usize = 64;

pub const SCALED_HEIGHT: usize = 320;
pub const SCALED_WIDTH: usize = 640;

pub struct Display {
    // vram
    data: [[u8; WIDTH]; HEIGHT],
    // indicate when to redraw
    needs_update: bool,
    window: Window,
    buffer: [u32; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        let window = Window::new(
            "Rusty CHIP-8 🦀",
            SCALED_WIDTH,
            SCALED_HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Display {
            data: [[0; WIDTH]; HEIGHT],
            needs_update: true,
            window: window,
            buffer: [0; WIDTH * HEIGHT],
        }
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
        self.needs_update = true;
    }

    pub fn build_buffer(&mut self) {
        let mut idx = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                self.buffer[idx] = if self.data[i][j] == 0 { 0 } else { 0x00FFFFFF };
                idx += 1;
            }
        }
    }

    pub fn update(&mut self) {
        if !self.window.is_open() {
            println!("Exiting game!");
            std::process::exit(1);
        }
        if self.needs_update {
            self.build_buffer();
            self.window
                .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
                .unwrap();
        }
        self.needs_update = false;
    }
}
