extern crate minifb;
use std::collections::HashMap;

use minifb::Key;

pub struct Keyboard {
    keys: [bool; 16],
    // key flags
    waiting_for_press: bool,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16],
            waiting_for_press: false,
        }
    }

    fn key_map() -> HashMap<Key, usize> {
        HashMap::from([
            (Key::Key1, 0x1),
            (Key::Key2, 0x2),
            (Key::Key3, 0x3),
            (Key::Key4, 0xC),
            (Key::Q, 0x4),
            (Key::W, 0x5),
            (Key::E, 0x6),
            (Key::R, 0xD),
            (Key::A, 0x7),
            (Key::S, 0x8),
            (Key::D, 0x9),
            (Key::F, 0xE),
            (Key::Z, 0xA),
            (Key::X, 0x0),
            (Key::C, 0xB),
            (Key::V, 0xF),
        ])
    }

    pub fn poll(&mut self, keys_pressed: Vec<Key>) -> usize {
        // poll every key and update keys
        self.keys = [false; 16];
        let mut found_press = 0xFF;

        keys_pressed.iter().for_each(|key| {
            if Keyboard::key_map().contains_key(key) {
                self.keys[Keyboard::key_map()[key]] = true;
                if found_press == 0xFF {
                    found_press = Keyboard::key_map()[key];
                }
            }
        });

        found_press
    }

    pub fn stop_waiting_for_press(&mut self) {
        self.waiting_for_press = false;
    }

    pub fn start_waiting_for_press(&mut self) {
        self.waiting_for_press = true;
    }

    pub fn is_waiting_for_press(&self) -> bool {
        self.waiting_for_press
    }

    pub fn query_key(&self, key_value: usize) -> bool {
        self.keys[key_value]
    }
}
