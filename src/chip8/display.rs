pub const HEIGHT: usize = 32;
pub const WIDTH: usize = 64;

pub struct Display {
    // vram
    data: [[u8; WIDTH]; HEIGHT],
    // indicate when to redraw
    needs_update: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            data: [[0; WIDTH]; HEIGHT],
            needs_update: false,
        }
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }

    pub fn will_need_update(&mut self) {
        self.needs_update = true;
    }
}
