const HEIGHT: usize = 32;
const WIDTH: usize = 64;

pub struct Display {
    data: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            data: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }
}
